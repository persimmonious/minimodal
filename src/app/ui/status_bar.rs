use super::text_window::TextWindowState;
use crate::app::{editor::Mode, theme::Theme};
use ratatui::{
    layout::{Constraint, Direction, Layout},
    prelude::{Buffer as TUI_Buffer, Rect},
    style::{Modifier, Style, Stylize},
    text::Span,
    widgets::{Block, Widget},
};
use std::rc::Weak;

pub struct StatusBar {
    line: usize,
    col: usize,
    percent: u8,
    mode: Mode,
    theme: Weak<Theme>,
}

impl StatusBar {
    pub fn new(win: &TextWindowState, mode: Mode, theme: Weak<Theme>) -> Self {
        let line = win.cursor.line;
        let total_lines = win.lines_count();
        let percent = if total_lines == 0 {
            0
        } else {
            (100.0 * line as f32 / (total_lines - 1) as f32).round() as u8
        };
        let col = win.cursor.col;
        StatusBar {
            line,
            col,
            percent,
            mode,
            theme,
        }
    }
}

impl Widget for &StatusBar {
    fn render(self, area: Rect, buf: &mut TUI_Buffer) {
        let theme = self.theme.upgrade().expect("referencing a dead theme!");
        let mode_span = match self.mode {
            Mode::Normal => Span::styled(" NORMAL ", theme.styles.status_mode_normal),
            Mode::Command => Span::styled(" COMMAND ", theme.styles.status_mode_command),
            Mode::Insert => Span::styled(" INSERT ", theme.styles.status_mode_insert),
        }
        .add_modifier(Modifier::BOLD);
        let mode_width = mode_span.width().try_into().expect("mode span too long!");
        let pos_span = Span::styled(
            format!("{}:{}", self.line + 1, self.col + 1),
            Style::default()
                .bg(theme.status_background)
                .fg(theme.status_foreground),
        );
        let pos_width = pos_span
            .width()
            .try_into()
            .expect("position span too long!");
        let percent_span = Span::styled(
            match self.percent {
                0 => "Top".to_owned(),
                100 => "Bot".to_owned(),
                _ => format!("{: >2}%", self.percent),
            },
            Style::default()
                .bg(theme.status_background)
                .fg(theme.status_foreground),
        );
        let percent_width = 3; // "Top", "Bot", or "XX%"

        let layout = Layout::new(
            Direction::Horizontal,
            vec![
                Constraint::Length(mode_width),
                Constraint::Fill(1),
                Constraint::Length(pos_width),
                Constraint::Length(1),
                Constraint::Length(percent_width),
                Constraint::Length(1),
                Constraint::Length(1),
            ],
        )
        .split(area);
        let mode_area = layout[0];
        let pos_area = layout[2];
        let percent_area = layout[4];
        let rightmost_padding = layout[6];

        Block::new().bg(theme.status_background).render(area, buf);
        mode_span.render(mode_area, buf);
        pos_span.render(pos_area, buf);
        percent_span.render(percent_area, buf);
        Block::new()
            .bg(match self.mode {
                Mode::Normal => theme.status_mode_normal_background,
                Mode::Command => theme.status_mode_command_background,
                Mode::Insert => theme.status_mode_insert_background,
            })
            .render(rightmost_padding, buf);
    }
}
