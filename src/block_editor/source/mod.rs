use ropey::Rope;
use std::borrow::Cow;

use crate::{block_editor::TextRange, lang::LanguageConfig, parse::TreeManager};
use edit_generation::*;
use undo_manager::{
    UndoManager,
    UndoStopCondition::{self, *},
};

mod edit_generation;
pub mod text_edit;
pub mod undo_manager;

pub use text_edit::TextEdit;

use super::{text_editor::selections::Selections, text_range::movement::TextMovement};

pub struct Source {
    /// the actual source text
    text: Rope,

    /// the language of the source
    pub language: &'static LanguageConfig,

    /// generates syntax tree from source code
    tree_manager: TreeManager,

    /// handles undo/redo
    undo_manager: UndoManager,

    /// pairs that were inserted and should be ignored on the next input
    input_ignore_stack: Vec<&'static str>,

    /// tracking which characters had pairs inserted with them, and should take
    /// the pair down with them if they are deleted
    paired_delete_stack: Vec<bool>,

    /// whether the text has changed since the last time it was checked
    text_changed: bool,
}

impl Source {
    pub fn new(text: Rope, language: &'static LanguageConfig) -> Self {
        let mut tree_manager = TreeManager::new(language);
        tree_manager.replace(&text);
        Self {
            text,
            language,
            tree_manager,
            undo_manager: UndoManager::new(),
            input_ignore_stack: Vec::new(),
            paired_delete_stack: Vec::new(),
            text_changed: true,
        }
    }

    pub fn text(&self) -> &Rope {
        &self.text
    }

    pub fn get_tree_cursor(&self) -> tree_sitter::TreeCursor {
        self.tree_manager.get_cursor()
    }

    /// Return if the text has changed since the last time this was called
    pub fn has_text_changed_since_last_check(&mut self) -> bool {
        let changed = self.text_changed;
        self.text_changed = false;
        changed
    }
}

/* ---------------------------------- edits --------------------------------- */
impl Source {
    /// Apply an edit to the source.
    fn apply_edit_helper(
        &mut self,
        edit: &TextEdit,
        undo_stop_before: UndoStopCondition,
        undo_stop_after: bool,
    ) {
        // update buffer and tree
        let undo = edit.apply(&mut self.text, &mut self.tree_manager);

        // update undo manager
        self.undo_manager
            .add_undo(&undo, edit, undo_stop_before, undo_stop_after);
        self.undo_manager.clear_redos();

        // mark that the text has changed
        self.text_changed = true;
    }

    /// Apply an edit to the source and selection.
    /// Notifies vscode depending on the origin property of the TextEdit.
    pub fn apply_edit(
        &mut self,
        edit: &TextEdit,
        undo_stop_before: UndoStopCondition,
        undo_stop_after: bool,
        selections: &mut Selections,
    ) {
        self.apply_edit_helper(edit, undo_stop_before, undo_stop_after);
        selections.set_selection(TextRange::new_cursor(edit.new_end()), self);
    }

    /// Handle inserting a string at the selection
    pub fn insert_str(&mut self, add: &str, selections: &mut Selections) {
        let old_selection = selections.selection().ordered();
        let edit = TextEdit::new(Cow::Borrowed(add), old_selection);
        self.apply_edit_helper(&edit, Always, true);
        selections.set_selection(TextRange::new_cursor(edit.new_end()), self)
    }

    /// Handle typing a single character
    /// (separate from `insert_str` because it also handles paired completion)
    pub fn insert_char(&mut self, add: &str, selections: &mut Selections) {
        let (edit, new_selection) = edit_for_insert_char(
            selections.selection(),
            &self.text,
            add,
            &mut self.input_ignore_stack,
            &mut self.paired_delete_stack,
        );

        if let Some(edit) = edit {
            self.apply_edit_helper(&edit, IfNotMerged, false);
        }

        selections.set_selection(new_selection, self);
    }

    pub fn insert_newline(&mut self, selections: &mut Selections) {
        let (edit, new_selection) = edit_for_insert_newline(
            selections.selection(),
            &self.text,
            self.language.new_scope_char,
        );
        self.apply_edit_helper(&edit, Always, false);

        selections.set_selection(new_selection, self);
    }

    pub fn delete(&mut self, movement: TextMovement, selections: &mut Selections) {
        let (edit, new_selection) = edit_for_delete(
            selections.selection(),
            &self.text,
            movement,
            selections.pseudo_selection(),
            &mut self.input_ignore_stack,
            &mut self.paired_delete_stack,
        );
        if let Some(edit) = edit {
            self.apply_edit_helper(&edit, IfNotMerged, false);
        }
        selections.set_selection(new_selection, self);
    }

    pub fn indent(&mut self, selections: &mut Selections) {
        let (edit, new_selection) = edit_for_indent(selections.selection(), &self.text);
        self.apply_edit_helper(&edit, Always, true);
        selections.set_selection(new_selection, self);
    }

    pub fn unindent(&mut self, selections: &mut Selections) {
        let (edit, new_selection) = edit_for_unindent(selections.selection(), &self.text);
        self.apply_edit_helper(&edit, Always, true);
        selections.set_selection(new_selection, self);
    }

    pub fn undo(&mut self, selections: &mut Selections) {
        if let Some(new_selection) = self
            .undo_manager
            .apply_undo(&mut self.text, &mut self.tree_manager)
        {
            self.text_changed = true;
            selections.set_selection(new_selection, self);
        }
    }

    pub fn redo(&mut self, selections: &mut Selections) {
        if let Some(new_selection) = self
            .undo_manager
            .apply_redo(&mut self.text, &mut self.tree_manager)
        {
            self.text_changed = true;
            selections.set_selection(new_selection, self);
        }
    }

    /// Clear the input ignore and pair delete stacks because the cursor is no longer in the same place and add a stop to the undo stack
    pub fn external_cursor_move(&mut self) {
        // clear input ignore stack
        self.input_ignore_stack.clear();
        self.paired_delete_stack.clear();

        // add a separator to the undo stack
        self.undo_manager.add_undo_stop()
    }
}
