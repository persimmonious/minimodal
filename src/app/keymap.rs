use super::Mode;
use crossterm::event::KeyCode;
use ratatui::crossterm::event::KeyEvent;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub enum EditorAction {
    EnterMenu,
    ExitEditor,
    ExitMenu,
}

#[derive(Debug)]
pub struct KeyMap {
    normal_mode: HashMap<KeyCode, EditorAction>,
    menu_mode: HashMap<KeyCode, EditorAction>,
    insert_mode: HashMap<KeyCode, EditorAction>,
}

impl KeyMap {
    pub fn handle_key(&self, key: &KeyEvent, mode: &Mode) -> Option<&EditorAction> {
        match mode {
            Mode::Menu => self.menu_mode.get(&key.code),
            Mode::Insert => self.insert_mode.get(&key.code),
            _ => self.normal_mode.get(&key.code),
        }
    }
}

impl Default for KeyMap {
    fn default() -> Self {
        let mut normal_mode = HashMap::new();
        let mut menu_mode = HashMap::new();
        let mut insert_mode = HashMap::new();
        normal_mode.insert(KeyCode::Char(' '), EditorAction::EnterMenu);
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
