use druid::Selector;

use crate::block_editor::text_editor::TextEdit;
use crate::lsp::{
    completion::VSCodeCompletionItem,
    diagnostics::{Diagnostic, VSCodeCodeAction},
    documentation::VSCodeHoverItem,
};

// set up
pub const SET_TEXT: Selector<String> = Selector::new("set_text");
pub const SET_FILE_NAME: Selector<String> = Selector::new("set_file_name");

// external edits
pub const APPLY_EDIT: Selector<TextEdit> = Selector::new("apply_edit");
pub const PASTE: Selector<String> = Selector::new("paste");

// lsp connection
pub const SET_DIAGNOSTICS: Selector<Vec<Diagnostic>> = Selector::new("set_diagnostics");
pub const SET_QUICK_FIX: Selector<Vec<VSCodeCodeAction>> = Selector::new("set_quick_fix");
pub const SET_COMPLETIONS: Selector<Vec<VSCodeCompletionItem>> = Selector::new("set_completions");
pub const SET_HOVER_DOCUMENTATION: Selector<Vec<VSCodeHoverItem>> = Selector::new("hover_info");

// internal communication
pub const DRAG_CANCELLED: Selector<()> = Selector::new("drag_cancelled");
