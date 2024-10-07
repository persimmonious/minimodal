use super::Mode;
use crossterm::event::KeyCode;
use ratatui::crossterm::event::KeyEvent;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub enum EditorAction {
    EnterInsert,
    EnterMenu,
    ExitEditor,
    ExitInsert,
    ExitMenu,
    InsertChar(char),
}

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
                Some(ref act) => Some((*act).clone())
            },

            _ => match self.normal_mode.get(&key.code) {
                None => None,
                Some(ref act) => Some((*act).clone())
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
        normal_mode.insert(KeyCode::Char(' '), EditorAction::EnterMenu);
        normal_mode.insert(KeyCode::Char('i'), EditorAction::EnterInsert);
        insert_mode.insert(KeyCode::Esc, EditorAction::ExitInsert);
        menu_mode.insert(KeyCode::Esc, EditorAction::ExitMenu);
        menu_mode.insert(KeyCode::Char(' '), EditorAction::ExitMenu);
        menu_mode.insert(KeyCode::Char('q'), EditorAction::ExitEditor);
        KeyMap {
            insert_mode,
            normal_mode,
            menu_mode,
        }
    }
}
