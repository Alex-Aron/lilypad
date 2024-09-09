use egui::{Painter, Response, Sense, Ui, Vec2, Widget};
use ropey::Rope;

use crate::{
    block_editor::{
        block_drawer::{self, Block},
        rope_ext::RopeExt,
        text_drawer::TextDrawer,
        MonospaceFont,
    },
    lang::LanguageConfig,
    parse::TreeManager,
    theme::blocks_theme::BlocksTheme,
};

pub struct LooseBlock {
    text: String,
    blocks: Vec<Block>,
    padding: Vec<f32>,
    min_size: Vec2,
    lang: &'static LanguageConfig,
    tree_manager: TreeManager,
    text_drawer: TextDrawer,
    interior_padding: f32,
}

impl LooseBlock {
    pub fn widget<'a>(
        &'a self,
        blocks_theme: BlocksTheme,
        font: &'a MonospaceFont,
    ) -> impl Widget + 'a {
        move |ui: &mut Ui| -> Response {
            let (id, rect) = ui.allocate_space(ui.available_size());
            let response = ui.interact(rect, id, Sense::click());
            self.draw(
                rect.min.to_vec2(),
                rect.width(),
                blocks_theme,
                font,
                ui.painter(),
            );

            // if mouse is hovering over the block, show grab cursor
            if response.hovered() {
                ui.ctx().set_cursor_icon(egui::CursorIcon::Grab);
            }

            response
        }
    }

    pub fn new(
        text: &str,
        interior_padding: f32,
        lang: &'static LanguageConfig,
        font: &MonospaceFont,
    ) -> Self {
        let mut block = Self {
            text: String::new(),
            blocks: vec![],
            padding: vec![],
            min_size: Vec2::ZERO,
            lang,
            tree_manager: TreeManager::new(lang),
            text_drawer: TextDrawer::new(lang),
            interior_padding,
        };
        block.set_text(text, font);
        block
    }

    fn set_text(&mut self, text: &str, font: &MonospaceFont) {
        self.text = text.to_string();
        let rope = Rope::from_str(text);

        self.tree_manager.replace(&rope);
        self.text_drawer
            .highlight(self.tree_manager.get_cursor().node(), &rope);

        // find blocks
        self.blocks =
            block_drawer::blocks_for_tree(&mut self.tree_manager.get_cursor(), &rope, self.lang);
        self.padding = block_drawer::make_padding(&self.blocks, rope.len_lines());

        // find dimensions
        let max_chars = rope.lines().map(|l| l.len_chars()).max().unwrap_or(0);
        let width = max_chars as f32 * font.size.x + self.interior_padding;
        let line_count = rope.len_lines() - if rope.ends_with('\n') { 1 } else { 0 };
        let height = (font.size.y * line_count as f32) + self.padding.iter().sum::<f32>();
        self.min_size = Vec2::new(width, height);
    }

    pub fn draw(
        &self,
        offset: Vec2,
        width: f32,
        blocks_theme: BlocksTheme,
        font: &MonospaceFont,
        painter: &Painter,
    ) {
        block_drawer::draw_blocks(
            &self.blocks,
            offset,
            width,
            None,
            blocks_theme,
            font,
            painter,
        );
        self.text_drawer
            .draw(&self.padding, offset, None, font, painter);
    }

    pub fn min_size(&self) -> Vec2 {
        self.min_size
    }

    pub fn text(&self) -> &str {
        &self.text
    }
}
