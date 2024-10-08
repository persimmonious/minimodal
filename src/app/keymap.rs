use super::{
    buffer::{
        HorizontalDirection::{self, *},
        RectilinearDirection::{self, *},
    },
    Mode,
};
use crossterm::event::KeyCode;
use ratatui::crossterm::event::KeyEvent;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub enum EditorAction {
    Append,
    AppendAtEOL,
    CycleTab(HorizontalDirection),
    EndOfBuffer,
    EnterInsert,
    EnterMenu,
    EOL,
    ExitEditor,
    ExitInsert,
    ExitMenu,
    Home,
    InsertChar(char),
    MoveToHomeAndEnterInsert,
    MoveCursor(Mode, RectilinearDirection),
    ReplaceLine,
}

use EditorAction::*;

#[derive(Debug)]
pub struct KeyMap {
    normal_mode: HashMap<KeyCode, EditorAction>,
    menu_mode: HashMap<KeyCode, EditorAction>,
    insert_mode: HashMap<KeyCode, EditorAction>,
}

impl KeyMap {
    pub fn handle_key(&self, key: &KeyEvent, mode: &Mode) -> Option<EditorAction> {
        match mode {
            Mode::Insert => self.handle_insert_mode(key),

            Mode::Menu => match self.menu_mode.get(&key.code) {
                None => None,
                Some(ref act) => Some((*act).clone()),
            },

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
        let mut menu_mode = HashMap::new();
        let mut insert_mode = HashMap::new();
        normal_mode.insert(KeyCode::Char(' '), EnterMenu);
        normal_mode.insert(KeyCode::Char('i'), EnterInsert);
        normal_mode.insert(KeyCode::Char('I'), MoveToHomeAndEnterInsert);
        normal_mode.insert(KeyCode::Char('S'), ReplaceLine);
        normal_mode.insert(KeyCode::Tab, CycleTab(Forwards));
        normal_mode.insert(KeyCode::BackTab, CycleTab(Backwards));
        normal_mode.insert(KeyCode::Char('h'), MoveCursor(Mode::Normal, Left));
        normal_mode.insert(KeyCode::Char('j'), MoveCursor(Mode::Normal, Down));
        normal_mode.insert(KeyCode::Char('k'), MoveCursor(Mode::Normal, Up));
        normal_mode.insert(KeyCode::Char('l'), MoveCursor(Mode::Normal, Right));
        normal_mode.insert(KeyCode::Char('$'), EOL);
        normal_mode.insert(KeyCode::Char('0'), Home);
        normal_mode.insert(KeyCode::Char('G'), EndOfBuffer);
        normal_mode.insert(KeyCode::Char('a'), Append);
        normal_mode.insert(KeyCode::Char('A'), AppendAtEOL);
        insert_mode.insert(KeyCode::Esc, ExitInsert);
        menu_mode.insert(KeyCode::Esc, ExitMenu);
        menu_mode.insert(KeyCode::Char(' '), ExitMenu);
        menu_mode.insert(KeyCode::Char('q'), ExitEditor);
        KeyMap {
            insert_mode,
            normal_mode,
            menu_mode,
        }
    }
}
