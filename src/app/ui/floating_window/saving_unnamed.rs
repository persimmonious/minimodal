use crossterm::event::{KeyCode, KeyEvent};
use ratatui::{
    buffer::Buffer as TUI_Buffer,
    layout::{Margin, Rect},
    style::{Color, Stylize},
    widgets::{Block, Borders, Paragraph, Widget},
};

use crate::app::editor::Editor;

use super::{EditorCallback, FloatingContent};

#[derive(Clone)]
pub(crate) struct SavingUnnamed();

impl FloatingContent for SavingUnnamed {
    fn handle_input(&mut self, input: &KeyEvent) -> Option<EditorCallback> {
        if matches!(input.code, KeyCode::Enter) {
            Some(Box::new(|ed: &mut Editor| ed.clear_floating_window()))
        } else {
            None
        }
    }

    fn render(&self, area: &Rect, buf: &mut TUI_Buffer) {
        Block::new()
            .fg(Color::LightYellow)
            .bg(Color::Black)
            .borders(Borders::all())
            .border_type(ratatui::widgets::BorderType::Rounded)
            .border_style(Color::DarkGray)
            .render(area.to_owned(), buf);
        Paragraph::new("Cannot save unnamed file!")
            .centered()
            .render(area.to_owned().inner(Margin::new(1, 1)), buf);
    }

    fn clone_as_box(&self) -> Box<dyn FloatingContent> {
        Box::new(SavingUnnamed())
    }
}
