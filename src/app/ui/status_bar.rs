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
    percent: u8,
    mode: Mode,
}

impl StatusBar {
    pub fn new(win: &TextWindowState, mode: Mode) -> Self {
        let line = win.cursor.line;
        let total_lines = win.lines_count();
        let percent = if total_lines == 0 {
            0
        } else {
            (100.0 * line as f32 / (total_lines - 1) as f32).round() as u8
        };
        let col = win.cursor.col;
        return StatusBar {
            line,
            col,
            percent,
            mode,
        };
    }
}

impl Widget for &StatusBar {
    fn render(self, area: Rect, buf: &mut TUI_Buffer) {
        let mode_span = Span::from(match self.mode {
            Mode::Normal => " NORMAL ",
            Mode::Command => " COMMAND ",
        });
        let mode_width = mode_span.width().try_into().expect("mode span too long!");
        let pos_span = Span::from(format!("{}:{}", self.line + 1, self.col + 1));
        let pos_width = pos_span
            .width()
            .try_into()
            .expect("position span too long!");
        let percent_span = Span::from(format!("{: >2}%", self.percent));
        let percent_width = 3;

        let layout = Layout::new(
            Direction::Horizontal,
            vec![
                Constraint::Length(mode_width),
                Constraint::Fill(1),
                Constraint::Length(pos_width),
                Constraint::Length(1),
                Constraint::Length(percent_width),
                Constraint::Length(1),
            ],
        )
        .split(area);
        let mode_area = layout[0];
        let middle_area = layout[1];
        let pos_area = layout[2];
        let pos_right_pad = layout[3];
        let percent_area = layout[4];
        let rightmost_padding = layout[5];

        mode_span.render(mode_area, buf);
        Block::new().render(middle_area, buf);
        pos_span.render(pos_area, buf);
        Block::new().render(pos_right_pad, buf);
        percent_span.render(percent_area, buf);
        Block::new().render(rightmost_padding, buf);
    }
}
