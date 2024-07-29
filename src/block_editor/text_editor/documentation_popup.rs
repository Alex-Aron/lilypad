use crate::block_editor::text_range::{TextPoint, TextRange};

use crate::vscode;
use crate::{
    block_editor::{commands, rope_ext::RopeSliceExt, EditorModel},
    lsp::documentation::{Documentation},
};
use druid::{
    Event, PaintCtx, Size, Widget,
};
use ropey::Rope;

pub struct DocumentationPopup {
    message: String,
    cursor: TextPoint,
}

impl DocumentationPopup {
    pub fn new() -> Self {
        DocumentationPopup {
            message: String::from(" "),
            cursor: TextPoint::new(0, 0),
        }
    }
    pub fn request_hover_info(&mut self, source: &Rope, selection: TextRange) {
        // only request completions when selection is just a cursor
        if !selection.is_cursor() {
            return;
        }
        let cursor = selection.start;

        // if we are at the start of the line, do not do anything
        // return does not trigger, but backspace does
        if source.line(cursor.line).whitespace_at_start() == cursor.col {
            return;
        }

        // set the cursor so we can filter results using it
        self.cursor = cursor;
        vscode::request_hover_info(cursor.line, cursor.col)
    }
}

impl Widget<EditorModel> for DocumentationPopup {
    fn event(
        &mut self,
        ctx: &mut druid::EventCtx,
        event: &druid::Event,
        _data: &mut EditorModel,
        _env: &druid::Env,
    ) {
        match event {
            Event::MouseMove(_) => {
                ctx.set_handled();
            }
            Event::Command(command) => {
                if let Some(_fixes) = command.get(commands::SET_HOVER_DOCUMENTATION) {
                    // TODO: verify id matches

                    ctx.request_layout();
                    ctx.request_paint();
                    ctx.set_handled();
                }
            }

            _ => {}
        }
    }

    fn lifecycle(
        &mut self,
        _ctx: &mut druid::LifeCycleCtx,
        _event: &druid::LifeCycle,
        _data: &EditorModel,
        _env: &druid::Env,
    ) {
    }

    fn update(
        &mut self,
        _ctx: &mut druid::UpdateCtx,
        _old_data: &EditorModel,
        _data: &EditorModel,
        _env: &druid::Env,
    ) {
    }

    fn layout(
        &mut self,
        _ctx: &mut druid::LayoutCtx<'_, '_>,
        _bc: &druid::BoxConstraints,
        _data: &EditorModel,
        _env: &druid::Env,
    ) -> Size {
        druid::Size::ZERO
    }

    fn paint(&mut self, _ctx: &mut PaintCtx, _data: &EditorModel, _env: &druid::widget::prelude::Env) {
    }
}

impl Documentation {
    pub fn draw(&self, _padding: &[f64], _source: &Rope, _ctx: &mut PaintCtx) {
        // let range = self.range.ordered();
        // let line_ranges = range.individual_lines(source);

        // let mut total_padding: f64 = padding.iter().take(range.start.line).sum();

        // for line_range in line_ranges {
        //     let line_num = line_range.start.line;

        //     total_padding += padding[line_num];

        //     // find bottom of current line
        //     let y =
        //         total_padding + ((line_num + 1) as f64 * FONT_HEIGHT.get().unwrap()) + OUTER_PAD;

        //     // find the start and end of the line
        //     let x = TOTAL_TEXT_X_OFFSET + (line_range.start.col as f64 * FONT_WIDTH.get().unwrap());
        //     let width =
        //         (line_range.end.col - line_range.start.col) as f64 * FONT_WIDTH.get().unwrap();

        //     // draw line
        //     let line = druid::kurbo::Line::new((x, y), (x + width, y));
        //     ctx.stroke(line, &self.severity.color(), 2.0);
        //Example above, implement soon.
    }
}
