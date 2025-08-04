pub mod floating_window;
pub mod leader_menu;
mod line_numbers;
pub mod status_bar;
pub mod text_window;

use super::{buffer::Buffer, editor::Mode, theme::Theme};
use ratatui::{
    buffer::Buffer as TUI_Buffer,
    layout::{Position, Rect},
    widgets::StatefulWidget,
};
use std::{
    cell::RefCell,
    rc::{Rc, Weak},
};
use text_window::{TextWindow, TextWindowState};

#[derive(Debug, Clone)]
pub struct Tab {}

#[derive(Debug)]
pub struct TabState {
    pub window_states: TextWindowState,
    pub buffer: Rc<RefCell<Buffer>>,
    pub windows: TextWindow,
    pub mode: Mode,
    current_window: usize,
}

impl TabState {
    pub fn new(buf: Buffer, theme: Weak<Theme>, mode: Mode) -> Self {
        let buf_rc = Rc::new(RefCell::new(buf));
        TabState {
            buffer: Rc::clone(&buf_rc),
            window_states: TextWindowState::new(Rc::downgrade(&buf_rc), mode.clone()),
            windows: TextWindow::new(Rc::downgrade(&buf_rc), theme.clone()),
            current_window: 0,
            mode,
        }
    }

    pub fn get_cursor_pos(&self) -> Position {
        self.window_states.get_cursor_pos()
    }

    pub fn propagate_mode(&mut self, mode: Mode) {
        self.window_states.set_mode(mode);
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
