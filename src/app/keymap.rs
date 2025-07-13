use super::{
    buffer::{
        HorizontalDirection::*, RectilinearDirection as Rectilinear, VerticalDirection as Vertical,
    },
    editor::{actions::EditorAction, Mode},
};
use crossterm::event::KeyCode;
use ratatui::crossterm::event::KeyEvent;
use std::collections::HashMap;

use crate::app::EditorAction::*;

#[derive(Debug)]
pub struct KeyMap {
    normal_mode: HashMap<KeyCode, EditorAction>,
    insert_mode: HashMap<KeyCode, EditorAction>,
    root_menu: HashMap<KeyCode, EditorAction>,
}

impl KeyMap {
    pub fn handle_key(&self, key: &KeyEvent, mode: &Mode) -> Option<EditorAction> {
        match mode {
            Mode::Insert => self.handle_insert_mode(key),

            Mode::Menu(submenu) => {
                let menu: &HashMap<KeyCode, EditorAction> = &self.root_menu;
                menu.get(&key.code).cloned()
            }

            _ => self.normal_mode.get(&key.code).cloned(),
        }
    }

    fn handle_insert_mode(&self, key: &KeyEvent) -> Option<EditorAction> {
        match key.code {
            KeyCode::Char(c) => Some(EditorAction::InsertChar(c)),
            _ => self.insert_mode.get(&key.code).cloned(),
        }
    }
}

impl Default for KeyMap {
    fn default() -> Self {
        let mut normal_mode = HashMap::new();
        let mut root_menu = HashMap::new();
        let mut insert_mode = HashMap::new();
        normal_mode.insert(KeyCode::Char(' '), EnterMenu);
        normal_mode.insert(KeyCode::Char('i'), EnterInsert);
        normal_mode.insert(KeyCode::Char('I'), MoveToHomeAndEnterInsert);
        normal_mode.insert(KeyCode::Char('S'), ReplaceLine);
        normal_mode.insert(KeyCode::Tab, CycleTab(Forward));
        normal_mode.insert(KeyCode::BackTab, CycleTab(Backward));
        normal_mode.insert(
            KeyCode::Char('h'),
            MoveCursor(Mode::Normal, Rectilinear::Left),
        );
        normal_mode.insert(KeyCode::Left, MoveCursor(Mode::Normal, Rectilinear::Left));
        normal_mode.insert(
            KeyCode::Char('j'),
            MoveCursor(Mode::Normal, Rectilinear::Down),
        );
        normal_mode.insert(KeyCode::Down, MoveCursor(Mode::Normal, Rectilinear::Down));
        normal_mode.insert(
            KeyCode::Char('k'),
            MoveCursor(Mode::Normal, Rectilinear::Up),
        );
        normal_mode.insert(KeyCode::Up, MoveCursor(Mode::Normal, Rectilinear::Up));
        normal_mode.insert(
            KeyCode::Char('l'),
            MoveCursor(Mode::Normal, Rectilinear::Right),
        );
        normal_mode.insert(KeyCode::Right, MoveCursor(Mode::Normal, Rectilinear::Right));
        normal_mode.insert(KeyCode::Char('$'), EOL);
        normal_mode.insert(KeyCode::Char('0'), Home);
        normal_mode.insert(KeyCode::Char('G'), EndOfBuffer);
        normal_mode.insert(KeyCode::Char('a'), Append);
        normal_mode.insert(KeyCode::Char('A'), AppendAtEOL);
        normal_mode.insert(KeyCode::Char('o'), InsertNewLine(Vertical::Down));
        normal_mode.insert(KeyCode::Char('O'), InsertNewLine(Vertical::Up));
        normal_mode.insert(KeyCode::Char('x'), RemoveChar(Forward));
        normal_mode.insert(KeyCode::Char('X'), RemoveChar(Backward));
        normal_mode.insert(KeyCode::Enter, NextLine);
        normal_mode.insert(KeyCode::Backspace, Back);
        insert_mode.insert(KeyCode::Esc, ExitInsert);
        insert_mode.insert(KeyCode::Enter, InsertLineBreak);
        insert_mode.insert(KeyCode::Delete, RemoveChar(Forward));
        insert_mode.insert(KeyCode::Backspace, RemoveChar(Backward));
        root_menu.insert(KeyCode::Esc, ExitMenu);
        root_menu.insert(KeyCode::Char(' '), ExitMenu);
        root_menu.insert(KeyCode::Char('q'), ExitEditor);
        root_menu.insert(KeyCode::Char('w'), SaveBuffer);
        KeyMap {
            insert_mode,
            normal_mode,
            root_menu,
        }
    }
}
