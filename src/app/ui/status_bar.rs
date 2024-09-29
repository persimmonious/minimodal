use ratatui::{
    layout::{Constraint, Direction, Layout},
    prelude::{Buffer as TUI_Buffer, Rect},
    text::Span,
    widgets::{Block, Widget},
};

use crate::app::Mode;

use super::text_window::TextWindowState;

pub struct StatusBar {
    line: usize,
    col: usize,
    mode: Mode,
}

impl StatusBar {
    pub fn new(win: &TextWindowState, mode: Mode) -> Self {
        let line = win.cursor.line;
        let col = win.cursor.col;
        return StatusBar { line, col, mode };
    }
}

impl Widget for &StatusBar {
    fn render(self, area: Rect, buf: &mut TUI_Buffer) {
        let mode_span = Span::from(match self.mode {
            Mode::Normal => " NORMAL ",
            Mode::Command => " COMMAND ",
        });
        let mode_width = mode_span.width().try_into().expect("mode span too long!");
        let pos_span = Span::from(format!("{}:{}", self.line, self.col));
        let pos_width = pos_span
            .width()
            .try_into()
            .expect("position span too long!");

        let layout = Layout::new(
            Direction::Horizontal,
            vec![
                Constraint::Length(mode_width),
                Constraint::Fill(1),
                Constraint::Length(pos_width),
                Constraint::Length(1),
            ],
        )
        .split(area);
        let mode_area = layout[0];
        let middle_area = layout[1];
        let pos_area = layout[2];
        let rightmost_padding = layout[3];

        mode_span.render(mode_area, buf);
        Block::new().render(middle_area, buf);
        pos_span.render(pos_area, buf);
        Block::new().render(rightmost_padding, buf);
    }
}
