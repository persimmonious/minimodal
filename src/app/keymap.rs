use ratatui::crossterm::event::KeyEvent;
use std::collections::HashMap;

#[derive(Debug)]
pub enum EditorAction {}

#[derive(Debug)]
pub struct KeyMap {
    normal_mode: HashMap<KeyEvent, EditorAction>,
}

impl KeyMap {}

impl Default for KeyMap {
    fn default() -> Self {
        let mut normal_mode = HashMap::new();
        KeyMap { normal_mode }
    }
}
