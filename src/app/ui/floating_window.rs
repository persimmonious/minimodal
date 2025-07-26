use std::rc::Rc;

use crossterm::event::KeyEvent;
use ratatui::{layout::Rect, Frame};

use crate::app::{editor::Editor, theme::Theme};

pub(crate) mod saving_unnamed;

type EditorCallback = Box<dyn FnOnce(&mut Editor)>;

pub trait FloatingContent {
    fn handle_input(&mut self, input: &KeyEvent) -> Option<EditorCallback>;

    fn render(&self, area: &Rect, buf: &mut Frame, theme: Rc<Theme>);

    fn clone_as_box(&self) -> Box<dyn FloatingContent>;
}

impl Clone for Box<dyn FloatingContent> {
    fn clone(&self) -> Self {
        self.clone_as_box()
    }
}
