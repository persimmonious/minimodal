pub mod leader_menu;
mod line_numbers;
pub mod status_bar;
pub mod text_window;
use super::{
    buffer::{Buffer, RectilinearDirection},
    theme::Theme,
};
use ratatui::{buffer::Buffer as TUI_Buffer, layout::Rect, widgets::StatefulWidget};
use std::{
    cell::RefCell,
    io,
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
    current_window: usize,
}

impl TabState {
    pub fn new(buf: Buffer, theme: Weak<Theme>) -> Self {
        let buf_rc = Rc::new(RefCell::new(buf));
        return TabState {
            buffer: Rc::clone(&buf_rc),
            window_states: TextWindowState::new(Rc::downgrade(&buf_rc), theme.clone()),
            windows: TextWindow::new(Rc::downgrade(&buf_rc), theme.clone()),
            current_window: 0,
        };
    }

    pub fn save_buffer(&self) -> io::Result<()> {
        self.buffer.borrow().save()?;
        Ok(())
    }

    pub fn get_cursor_pos(&self) -> (usize, usize) {
        return self.window_states.get_cursor_pos();
    }

    pub fn insert_char(&mut self, c: char) {
        let current_pos = &self.window_states.cursor;
        self.buffer.borrow_mut().insert_char(c, current_pos);
        self.window_states.advance_insertion_cursor();
    }

    pub fn replace_line(&mut self) {
        let current_pos = &self.window_states.cursor;
        self.buffer.borrow_mut().replace_line(current_pos);
        self.window_states.snap_to_EOL();
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
