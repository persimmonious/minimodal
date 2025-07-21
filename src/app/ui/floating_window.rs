use crossterm::event::KeyEvent;
use ratatui::{buffer::Buffer as TUI_Buffer, layout::Rect};

use crate::app::editor::Editor;

pub trait FloatingContent {
    fn handle_input(&mut self, input: &KeyEvent) -> Option<Box<fn(&mut Editor)>>;

    fn render(&self, area: &Rect, buf: &mut TUI_Buffer);
}
