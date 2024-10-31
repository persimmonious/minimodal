use crate::app::{
    buffer::{
        BufferPosition, HorizontalDirection as Horizontal, RectilinearDirection as Rectilinear,
        VerticalDirection,
    },
    ui::leader_menu::SubMenu,
};

use super::{actions::EditorAction, Editor, Mode};

impl Editor {
    pub fn execute_editor_action(&mut self, action: EditorAction) {
        match action {
            EditorAction::Append => self.append(),
            EditorAction::AppendAtEOL => {
                self.sticky_jump_to_EOL();
                self.append();
            }
            EditorAction::EnterInsert => self.enter_insert(),
            EditorAction::EnterMenu => self.enter_menu(),
            EditorAction::ExitInsert => self.exit_insert(),
            EditorAction::ExitEditor => self.exit(),
            EditorAction::ExitMenu => self.exit_menu(),
            EditorAction::InsertChar(c) => self.insert_char(c),
            EditorAction::MoveToHomeAndEnterInsert => {
                self.jump_to_home();
                self.enter_insert();
            }
            EditorAction::ReplaceLine => self.replace_line(),
            EditorAction::CycleTab(dir) => self.cycle_tab(dir),
            EditorAction::MoveCursor(mode, dir) => match mode {
                Mode::Normal => self.move_cursor(dir),
                _ => (),
            },
            EditorAction::EOL => self.sticky_jump_to_EOL(),
            EditorAction::Home => self.jump_to_home(),
            EditorAction::EndOfBuffer => self.jump_to_last_line(),
            EditorAction::SaveBuffer => self.save_current_buffer(),
            EditorAction::InsertNewLine(dir) => self.insert_new_line(dir),
            EditorAction::RemoveChar => self.remove_char(),
            EditorAction::InsertLineBreak => self.insert_line_break(),
            EditorAction::NextLine => self.jump_to_next_line(),
            EditorAction::Back => self.back(),
        }
    }

    fn enter_menu(&mut self) {
        self.mode = Mode::Menu(SubMenu::Root);
    }

    fn exit_menu(&mut self) {
        self.mode = Mode::Normal;
    }

    fn enter_insert(&mut self) {
        self.current_winstate().stick_to_EOL = false;
        self.mode = Mode::Insert;
    }

    fn exit_insert(&mut self) {
        self.mode = Mode::Normal;
        self.current_winstate().snap_to_EOL();
    }

    fn insert_char(&mut self, c: char) {
        let cursor = self.current_winstate().cursor.clone();
        self.current_tabstate_mut()
            .buffer
            .borrow_mut()
            .insert_char(c, &cursor);
        self.current_winstate().advance_insertion_cursor();
    }

    fn remove_char(&mut self) {
        let BufferPosition { line, col } = self.current_winstate().cursor;
        let mut buffer = self.current_tabstate_mut().buffer.borrow_mut();
        match buffer.line_length(line) {
            None => return,
            Some(len) => {
                if col >= len {
                    return;
                }
                buffer.lines[line].remove(col);
                drop(buffer);
                self.current_winstate().snap_to_EOL();
            }
        }
    }

    fn replace_line(&mut self) {
        self.enter_insert();
        let current_pos = self.current_winstate().cursor.clone();
        self.current_tabstate_mut()
            .buffer
            .borrow_mut()
            .clear_line(&current_pos);
        self.current_winstate().snap_to_EOL();
    }

    fn append(&mut self) {
        self.enter_insert();
        if !self.current_winstate().cursor_at_EOL() {
            self.current_winstate().advance_insertion_cursor();
        }
    }

    fn insert_new_line(&mut self, dir: VerticalDirection) {
        let line_count = self.current_tabstate_mut().buffer.borrow().lines_count();
        let mut line = self.current_winstate().cursor.line;
        if let VerticalDirection::Down = dir {
            line += 1;
        }
        let mut buffer = self.current_tabstate_mut().buffer.borrow_mut();

        if line_count == 0 {
            buffer.add_line(0, "".to_string());
            buffer.add_line(1, "".to_string());
            drop(buffer);
            if let VerticalDirection::Down = dir {
                let second_line = BufferPosition { line, col: 0 };
                self.current_winstate().jump(&second_line);
            }
            return;
        }

        buffer.add_line(line, "".to_string());
        drop(buffer);
        if let VerticalDirection::Down = dir {
            let second_line = BufferPosition { line, col: 0 };
            self.current_winstate().jump(&second_line);
        } else {
            self.current_winstate().jump_to_home();
        }
        self.enter_insert();
    }

    fn insert_line_break(&mut self) {
        let cursor = self.current_winstate().cursor.clone();
        self.current_tabstate_mut()
            .buffer
            .borrow_mut()
            .split_line(&cursor);
        let new_pos = BufferPosition {
            line: cursor.line + 1,
            col: 0,
        };
        self.current_winstate().jump(&new_pos);
    }

    fn save_current_buffer(&mut self) {
        self.current_tabstate_mut().buffer.borrow().save().unwrap();
        self.exit_menu();
    }

    fn exit(&mut self) {
        self.active = false;
    }

    fn cycle_tab(&mut self, dir: Horizontal) {
        self.current_tab = match dir {
            Horizontal::Forwards => (self.current_tab + 1) % self.tabs.len(),
            Horizontal::Backwards => match self.current_tab {
                0 => self.tabs.len() - 1,
                current => (current - 1) % self.tabs.len(),
            },
        }
    }

    fn move_cursor(&mut self, dir: Rectilinear) {
        self.tab_states[self.current_tab]
            .window_states
            .move_cursor(dir);
    }

    fn jump_to_EOL(&mut self) {
        self.current_winstate().jump_to_EOL();
    }

    fn sticky_jump_to_EOL(&mut self) {
        self.tab_states[self.current_tab]
            .window_states
            .sticky_jump_to_EOL();
    }

    fn jump_to_home(&mut self) {
        self.tab_states[self.current_tab]
            .window_states
            .jump_to_home();
    }

    fn jump_to_last_line(&mut self) {
        self.tab_states[self.current_tab]
            .window_states
            .jump_to_last_line();
    }

    fn jump_to_next_line(&mut self) {
        let mut cursor = self.current_winstate().cursor.clone();
        cursor.line += 1;
        if cursor.line < self.current_winstate().lines_count() {
            cursor.col = 0;
            self.current_winstate().jump(&cursor);
            self.current_winstate().stick_to_EOL = false;
        }
    }

    fn back(&mut self) {
        let cursor = self.current_winstate().cursor.clone();
        if cursor.col > 0 {
            self.move_cursor(Rectilinear::Left);
        } else if cursor.line > 0 {
            self.move_cursor(Rectilinear::Up);
            self.jump_to_EOL();
        }
    }
}
