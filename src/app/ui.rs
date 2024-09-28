mod line_numbers;
mod text_window;
use super::{buffer::Buffer, theme::Theme};
use text_window::{TextWindow, TextWindowState};
use ratatui::{buffer::Buffer as TUI_Buffer, layout::Rect, widgets::StatefulWidget};
use std::rc::{Rc, Weak};

#[derive(Debug, Clone)]
pub struct Tab {}

#[derive(Debug)]
pub struct TabState {
    pub window_states: TextWindowState,
    pub buffer: Rc<Buffer>,
    pub windows: TextWindow,
    current_window: usize,
}

impl TabState {
    pub fn new(buf: Buffer, theme: Weak<Theme>) -> Self {
        let buf_rc = Rc::new(buf);
        return TabState {
            buffer: Rc::clone(&buf_rc),
            window_states: TextWindowState::new(Rc::downgrade(&buf_rc), theme.clone()),
            windows: TextWindow::new(Rc::downgrade(&buf_rc), theme.clone()),
            current_window: 0,
        };
    }
}

impl Tab {
    pub fn new() -> Self {
        Tab {}
    }
}

impl StatefulWidget for Tab {
    type State = TabState;

    fn render(self, area: Rect, buf: &mut TUI_Buffer, state: &mut Self::State) {
        state
            .windows
            .clone()
            .render(area, buf, &mut state.window_states);
    }
}
