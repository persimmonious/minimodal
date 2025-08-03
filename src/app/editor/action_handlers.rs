use crate::app::{
    buffer::{
        BufferPosition, HorizontalDirection as Horizontal, RectilinearDirection as Rectilinear,
        VerticalDirection,
    },
    cleanup::CleanUnwrap,
    ui::{
        floating_window::{saving_unnamed::SavingUnnamed, FloatingContent},
        leader_menu::SubMenu,
        text_window::selection::Selection,
    },
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
            EditorAction::EnterFloatingMenu(menu) => self.enter_floating_menu(menu),
            EditorAction::EnterSelect => self.enter_select(),
            EditorAction::ExitInsert => self.exit_insert(),
            EditorAction::ExitEditor => self.exit(),
            EditorAction::ExitMenu => self.exit_menu(),
            EditorAction::ExitSelect => self.exit_select(),
            EditorAction::InsertChar(c) => self.insert_char(c),
            EditorAction::MoveToHomeAndEnterInsert => {
                self.jump_to_home();
                self.enter_insert();
            }
            EditorAction::ReplaceLine => self.replace_line(),
            EditorAction::CycleTab(dir) => self.cycle_tab(dir),
            EditorAction::MoveCursor(mode, dir) => self.move_cursor(&mode, dir),
            EditorAction::EOL => self.sticky_jump_to_EOL(),
            EditorAction::Home => self.jump_to_home(),
            EditorAction::EndOfBuffer => self.jump_to_last_line(),
            EditorAction::SaveBuffer => self.save_current_buffer(),
            EditorAction::InsertNewLine(dir) => self.insert_new_line(dir),
            EditorAction::RemoveChar(dir) => self.remove_char(dir),
            EditorAction::InsertLineBreak => self.insert_line_break(),
            EditorAction::NextLine => self.jump_to_next_line(),
            EditorAction::Back => self.back(),
        }
    }

    fn enter_menu(&mut self) {
        self.lower_menu = Some(SubMenu::Root);
    }

    fn enter_floating_menu(&mut self, menu: Box<dyn FloatingContent>) {
        self.floating_window = Some(menu);
    }

    fn exit_menu(&mut self) {
        self.lower_menu = None;
    }

    fn enter_insert(&mut self) {
        self.current_winstate_mut().stick_to_EOL = false;
        self.mode = Mode::Insert;
    }

    fn exit_insert(&mut self) {
        self.mode = Mode::Normal;
        self.current_winstate_mut().snap_to_EOL();
        self.current_winstate_mut().last_manual_col = self.current_bufpos().col;
    }

    fn insert_char(&mut self, c: char) {
        let cursor = self.current_bufpos();
        self.current_buffer_mut().insert_char(c, &cursor);
        self.current_winstate_mut().advance_insertion_cursor();
    }

    fn remove_char(&mut self, dir: Horizontal) {
        let pos = self.current_bufpos();
        let mode = self.get_mode().to_owned();
        let len = self.current_buffer().line_length(pos.line).unwrap_or(0);
        let lines_count = self.current_buffer().lines_count();
        match (mode, dir) {
            (Mode::Normal, Horizontal::Forward) if pos.col < len => {
                self.current_buffer_mut().remove_char(&pos);
                self.current_winstate_mut().snap_to_EOL();
            }
            (Mode::Normal, Horizontal::Backward) if pos.col > 0 && pos.col < len => {
                self.current_buffer_mut().remove_char(&BufferPosition {
                    col: pos.col - 1,
                    ..pos
                });
                self.current_winstate_mut()
                    .move_cursor(&Mode::Normal, Rectilinear::Left);
            }
            (Mode::Insert, Horizontal::Forward) if pos.col <= len => {
                if pos.col < len {
                    self.current_buffer_mut().remove_char(&pos);
                } else if pos.col == len && pos.line + 1 < lines_count {
                    self.current_buffer_mut().join_with_next_line(pos.line);
                }
            }
            (Mode::Insert, Horizontal::Backward) if pos.col <= len => {
                if pos.col > 0 {
                    self.current_buffer_mut().remove_char(&BufferPosition {
                        col: pos.col - 1,
                        ..pos
                    });
                    self.current_winstate_mut()
                        .move_cursor(&Mode::Insert, Rectilinear::Left);
                } else if pos.line > 0 {
                    self.current_winstate_mut()
                        .move_cursor(&Mode::Insert, Rectilinear::Up);
                    self.current_winstate_mut().jump_past_EOL();
                    self.current_buffer_mut().join_with_next_line(pos.line - 1);
                }
            }
            (_, _) => (),
        }
        self.current_winstate_mut().last_manual_col = self.current_winstate().cursor.col;
    }

    fn replace_line(&mut self) {
        self.enter_insert();
        let current_pos = self.current_bufpos();
        self.current_buffer_mut().clear_line(&current_pos);
        self.current_winstate_mut().snap_to_EOL();
    }

    fn append(&mut self) {
        self.enter_insert();
        if !self.current_winstate().cursor_past_EOL() {
            self.current_winstate_mut().advance_insertion_cursor();
        }
    }

    fn insert_new_line(&mut self, dir: VerticalDirection) {
        let line_count = self.current_buffer().lines_count();
        let mut line = self.current_bufpos().line;
        if let VerticalDirection::Down = dir {
            line += 1;
        }
        let mut buffer = self.current_buffer_mut();

        if line_count == 0 {
            buffer.add_line(0, "".to_string());
            buffer.add_line(1, "".to_string());
            drop(buffer);
            if let VerticalDirection::Down = dir {
                let second_line = BufferPosition { line, col: 0 };
                self.current_winstate_mut().jump(&second_line);
            }
            return;
        }

        buffer.add_line(line, "".to_string());
        drop(buffer);
        if let VerticalDirection::Down = dir {
            let second_line = BufferPosition { line, col: 0 };
            self.current_winstate_mut().jump(&second_line);
        } else {
            self.current_winstate_mut().jump_to_home();
        }
        self.enter_insert();
    }

    fn insert_line_break(&mut self) {
        let cursor = self.current_bufpos();
        self.current_buffer_mut().split_line(&cursor);
        let new_pos = BufferPosition {
            line: cursor.line + 1,
            col: 0,
        };
        self.current_winstate_mut().jump(&new_pos);
    }

    fn save_current_buffer(&mut self) {
        if self.current_buffer().read_name().is_some() {
            self.current_buffer()
                .save()
                .clean_expect("Saving the buffer failed");
        } else {
            self.enter_floating_menu(Box::new(SavingUnnamed::default()));
        }
        self.exit_menu();
    }

    fn exit(&mut self) {
        self.active = false;
    }

    fn cycle_tab(&mut self, dir: Horizontal) {
        self.current_tab = match dir {
            Horizontal::Forward => (self.current_tab + 1) % self.tabs.len(),
            Horizontal::Backward => match self.current_tab {
                0 => self.tabs.len() - 1,
                current => (current - 1) % self.tabs.len(),
            },
        }
    }

    fn move_cursor(&mut self, mode: &Mode, dir: Rectilinear) {
        self.current_winstate_mut().move_cursor(mode, dir);
    }

    fn jump_to_EOL(&mut self) {
        self.current_winstate_mut().jump_to_EOL();
    }

    fn sticky_jump_to_EOL(&mut self) {
        self.current_winstate_mut().sticky_jump_to_EOL();
        if matches!(self.get_mode(), Mode::Insert) {
            self.current_winstate_mut().jump_past_EOL();
        }
    }

    fn jump_to_home(&mut self) {
        self.current_winstate_mut().jump_to_home();
    }

    fn jump_to_last_line(&mut self) {
        self.current_winstate_mut().jump_to_last_line();
    }

    fn jump_to_next_line(&mut self) {
        let mut cursor = self.current_bufpos();
        cursor.line += 1;
        if cursor.line < self.current_winstate_mut().lines_count() {
            cursor.col = 0;
            self.current_winstate_mut().jump(&cursor);
            self.current_winstate_mut().stick_to_EOL = false;
        }
    }

    fn back(&mut self) {
        let cursor = self.current_bufpos();
        let mode = self.get_mode().to_owned();
        if cursor.col > 0 {
            self.move_cursor(&mode, Rectilinear::Left);
        } else if cursor.line > 0 {
            self.move_cursor(&mode, Rectilinear::Up);
            self.jump_to_EOL();
        }
    }

    fn enter_select(&mut self) {
        self.mode = Mode::Select(Selection::from_single(&self.current_bufpos()));
    }

    fn exit_select(&mut self) {
        self.mode = Mode::Normal;
    }
}
