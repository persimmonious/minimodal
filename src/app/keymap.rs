use super::{
    actions::EditorAction,
    buffer::{
        HorizontalDirection::*, RectilinearDirection as Rectilinear, VerticalDirection as Vertical,
    },
    Mode,
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
                let menu: &HashMap<KeyCode, EditorAction> = match submenu {
                    _ => &self.root_menu,
                };
                match menu.get(&key.code) {
                    None => None,
                    Some(ref act) => Some((*act).clone()),
                }
            }

            _ => match self.normal_mode.get(&key.code) {
                None => None,
                Some(ref act) => Some((*act).clone()),
            },
        }
    }

    fn handle_insert_mode(&self, key: &KeyEvent) -> Option<EditorAction> {
        match key.code {
            KeyCode::Char(c) => Some(EditorAction::InsertChar(c)),
            _ => match self.insert_mode.get(&key.code) {
                None => None,
                Some(ref act) => Some((*act).clone()),
            },
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
        normal_mode.insert(KeyCode::Tab, CycleTab(Forwards));
        normal_mode.insert(KeyCode::BackTab, CycleTab(Backwards));
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
        normal_mode.insert(KeyCode::Char('x'), RemoveChar);
        insert_mode.insert(KeyCode::Esc, ExitInsert);
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
