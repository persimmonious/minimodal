use crossterm::event::KeyEvent;
use ratatui::widgets::Widget;

pub trait FloatingContent: Widget {
    fn handle_input(&mut self, input: &KeyEvent);
}
