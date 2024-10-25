pub mod leader_menu;
mod line_numbers;
pub mod status_bar;
pub mod text_window;
use super::{
    buffer::{Buffer, BufferPosition, VerticalDirection as Vertical},
    theme::Theme,
};
use ratatui::{buffer::Buffer as TUI_Buffer, layout::Rect, widgets::StatefulWidget};
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

    pub fn get_cursor_pos(&self) -> (usize, usize) {
        return self.window_states.get_cursor_pos();
    }

    pub fn lines_count(&self) -> usize {
        return self.buffer.borrow().lines_count();
    }

    pub fn line_length(&self, index: usize) -> Option<usize> {
        return self.buffer.borrow().line_length(index);
    }

    pub fn remove_char(&mut self) {
        let BufferPosition { line, col } = self.window_states.cursor;
        match self.line_length(line) {
            None => return,
            Some(len) => {
                if col >= len {
                    return;
                }
                self.buffer.borrow_mut().lines[line].remove(col);
                self.window_states.snap_to_EOL();
            }
        }
    }

    pub fn replace_line(&mut self) {
        let current_pos = &self.window_states.cursor;
        self.buffer.borrow_mut().clear_line(current_pos);
        self.window_states.snap_to_EOL();
    }

    pub fn insert_new_line(&mut self, dir: Vertical) {
        let line_count = self.buffer.borrow().lines.len();
        let mut line = self.window_states.cursor.line;
        if let Vertical::Down = dir {
            line += 1;
        }

        if line_count == 0 {
            self.buffer.borrow_mut().add_line(0, "".to_string());
            self.buffer.borrow_mut().add_line(1, "".to_string());
            if let Vertical::Down = dir {
                let second_line = BufferPosition { line, col: 0 };
                self.window_states.jump(&second_line);
            }
            return;
        }

        self.buffer.borrow_mut().add_line(line, "".to_string());
        if let Vertical::Down = dir {
            let second_line = BufferPosition { line, col: 0 };
            self.window_states.jump(&second_line);
        } else {
            self.window_states.jump_to_home();
        }
    }

    pub fn insert_line_break(&mut self) {
        let cursor = &self.window_states.cursor;
        self.buffer.borrow_mut().split_line(cursor);
        let new_pos = BufferPosition {
            line: cursor.line + 1,
            col: 0,
        };
        self.window_states.jump(&new_pos);
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
