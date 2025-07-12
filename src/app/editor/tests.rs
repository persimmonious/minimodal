use std::{ffi::OsString, str::FromStr};

use crossterm::event::KeyModifiers;

use super::*;

#[test]
fn test_create_unnamed_editor() {
    let editor = Editor::new(vec![Buffer::untitled()], Theme::default());
    assert_eq!(editor.current_bufpos(), BufferPosition { line: 0, col: 0 });
    assert_eq!(editor.mode, Mode::Normal);
    assert!(editor
        .current_tabstate()
        .buffer
        .borrow()
        .read_name()
        .is_none());
    assert!(editor.current_tabstate().buffer.borrow().path().is_none());
}

#[test]
fn test_create_named_editor() {
    let editor = Editor::new(
        vec![Buffer::empty(
            OsString::from_str("newfile.txt").unwrap(),
            OsString::from_str("newdir/newfile.txt").unwrap(),
        )],
        Theme::default(),
    );
    assert_eq!(editor.current_bufpos(), BufferPosition { line: 0, col: 0 });
    assert_eq!(editor.mode, Mode::Normal);
    assert_eq!(
        editor.current_tabstate().buffer.borrow().read_name(),
        Some(OsString::from_str("newfile.txt").unwrap().as_os_str())
    );
    assert_eq!(
        editor.current_tabstate().buffer.borrow().path(),
        Some(
            OsString::from_str("newdir/newfile.txt")
                .unwrap()
                .as_os_str()
        )
    );
}

#[test]
fn test_simple_one_line_input() {
    let mut editor = Editor::new(vec![Buffer::untitled()], Theme::default());
    editor.handle_key_press(KeyEvent::new(event::KeyCode::Char('i'), KeyModifiers::NONE));
    editor.handle_key_press(KeyEvent::new(event::KeyCode::Char('a'), KeyModifiers::NONE));
    editor.handle_key_press(KeyEvent::new(event::KeyCode::Char('b'), KeyModifiers::NONE));
    editor.handle_key_press(KeyEvent::new(event::KeyCode::Char('c'), KeyModifiers::NONE));
    editor.handle_key_press(KeyEvent::new(event::KeyCode::Char('d'), KeyModifiers::NONE));
    editor.handle_key_press(KeyEvent::new(event::KeyCode::Char('1'), KeyModifiers::NONE));
    editor.handle_key_press(KeyEvent::new(event::KeyCode::Char('2'), KeyModifiers::NONE));
    editor.handle_key_press(KeyEvent::new(event::KeyCode::Char('3'), KeyModifiers::NONE));
    editor.handle_key_press(KeyEvent::new(event::KeyCode::Char('!'), KeyModifiers::NONE));
    assert_eq!(editor.current_bufpos(), BufferPosition { line: 0, col: 8 });
    assert_eq!(editor.mode, Mode::Insert);
    editor.handle_key_press(KeyEvent::new(event::KeyCode::Esc, KeyModifiers::NONE));
    assert_eq!(editor.current_bufpos(), BufferPosition { line: 0, col: 7 });
    assert_eq!(editor.mode, Mode::Normal);
    assert_eq!(
        editor.current_tabstate().buffer.borrow().lines,
        &["abcd123!".to_owned()]
    );
}

#[test]
fn test_multi_line_input() {
    let mut editor = Editor::new(vec![Buffer::untitled()], Theme::default());
    editor.handle_key_press(KeyEvent::new(event::KeyCode::Char('i'), KeyModifiers::NONE));
    assert_eq!(editor.current_bufpos(), BufferPosition { line: 0, col: 0 });
    editor.handle_key_press(KeyEvent::new(event::KeyCode::Enter, KeyModifiers::NONE));
    assert_eq!(editor.current_bufpos(), BufferPosition { line: 1, col: 0 });
    editor.handle_key_press(KeyEvent::new(event::KeyCode::Char('H'), KeyModifiers::NONE));
    editor.handle_key_press(KeyEvent::new(event::KeyCode::Char('e'), KeyModifiers::NONE));
    editor.handle_key_press(KeyEvent::new(event::KeyCode::Char('l'), KeyModifiers::NONE));
    editor.handle_key_press(KeyEvent::new(event::KeyCode::Char('l'), KeyModifiers::NONE));
    editor.handle_key_press(KeyEvent::new(event::KeyCode::Char('o'), KeyModifiers::NONE));
    assert_eq!(editor.current_bufpos(), BufferPosition { line: 1, col: 5 });
    editor.handle_key_press(KeyEvent::new(event::KeyCode::Enter, KeyModifiers::NONE));
    assert_eq!(editor.current_bufpos(), BufferPosition { line: 2, col: 0 });
    editor.handle_key_press(KeyEvent::new(event::KeyCode::Char('W'), KeyModifiers::NONE));
    editor.handle_key_press(KeyEvent::new(event::KeyCode::Char('o'), KeyModifiers::NONE));
    editor.handle_key_press(KeyEvent::new(event::KeyCode::Char('r'), KeyModifiers::NONE));
    editor.handle_key_press(KeyEvent::new(event::KeyCode::Char('l'), KeyModifiers::NONE));
    editor.handle_key_press(KeyEvent::new(event::KeyCode::Char('d'), KeyModifiers::NONE));
    assert_eq!(editor.current_bufpos(), BufferPosition { line: 2, col: 5 });
    editor.handle_key_press(KeyEvent::new(event::KeyCode::Enter, KeyModifiers::NONE));
    assert_eq!(editor.current_bufpos(), BufferPosition { line: 3, col: 0 });
    editor.handle_key_press(KeyEvent::new(event::KeyCode::Char('!'), KeyModifiers::NONE));
    assert_eq!(editor.current_bufpos(), BufferPosition { line: 3, col: 1 });
    editor.handle_key_press(KeyEvent::new(event::KeyCode::Esc, KeyModifiers::NONE));
    assert_eq!(editor.current_bufpos(), BufferPosition { line: 3, col: 0 });
    assert_eq!(editor.mode, Mode::Normal);
    assert_eq!(
        editor.current_tabstate().buffer.borrow().lines,
        &["", "Hello", "World", "!"]
    );
}

#[test]
fn test_text_with_multiple_line_breaks() {
    let mut editor = Editor::new(vec![Buffer::untitled()], Theme::default());
    editor.handle_key_press(KeyEvent::new(event::KeyCode::Char('i'), KeyModifiers::NONE));
    assert_eq!(editor.current_bufpos(), BufferPosition { line: 0, col: 0 });
    editor.handle_key_press(KeyEvent::new(event::KeyCode::Char('a'), KeyModifiers::NONE));
    editor.handle_key_press(KeyEvent::new(event::KeyCode::Char('b'), KeyModifiers::NONE));
    editor.handle_key_press(KeyEvent::new(event::KeyCode::Char('c'), KeyModifiers::NONE));
    assert_eq!(editor.current_bufpos(), BufferPosition { line: 0, col: 3 });
    editor.handle_key_press(KeyEvent::new(event::KeyCode::Enter, KeyModifiers::NONE));
    assert_eq!(editor.current_bufpos(), BufferPosition { line: 1, col: 0 });
    editor.handle_key_press(KeyEvent::new(event::KeyCode::Char('d'), KeyModifiers::NONE));
    editor.handle_key_press(KeyEvent::new(event::KeyCode::Char('e'), KeyModifiers::NONE));
    editor.handle_key_press(KeyEvent::new(event::KeyCode::Char('f'), KeyModifiers::NONE));
    editor.handle_key_press(KeyEvent::new(event::KeyCode::Char('g'), KeyModifiers::NONE));
    editor.handle_key_press(KeyEvent::new(event::KeyCode::Char('h'), KeyModifiers::NONE));
    assert_eq!(editor.current_bufpos(), BufferPosition { line: 1, col: 5 });
    editor.handle_key_press(KeyEvent::new(event::KeyCode::Enter, KeyModifiers::NONE));
    assert_eq!(editor.current_bufpos(), BufferPosition { line: 2, col: 0 });
    editor.handle_key_press(KeyEvent::new(event::KeyCode::Enter, KeyModifiers::NONE));
    assert_eq!(editor.current_bufpos(), BufferPosition { line: 3, col: 0 });
    editor.handle_key_press(KeyEvent::new(event::KeyCode::Enter, KeyModifiers::NONE));
    assert_eq!(editor.current_bufpos(), BufferPosition { line: 4, col: 0 });
    editor.handle_key_press(KeyEvent::new(event::KeyCode::Char('i'), KeyModifiers::NONE));
    editor.handle_key_press(KeyEvent::new(event::KeyCode::Char('j'), KeyModifiers::NONE));
    editor.handle_key_press(KeyEvent::new(event::KeyCode::Char('k'), KeyModifiers::NONE));
    editor.handle_key_press(KeyEvent::new(event::KeyCode::Char('l'), KeyModifiers::NONE));
    editor.handle_key_press(KeyEvent::new(event::KeyCode::Esc, KeyModifiers::NONE));
    assert_eq!(editor.current_bufpos(), BufferPosition { line: 4, col: 3 });
    assert_eq!(editor.mode, Mode::Normal);
    assert_eq!(
        editor.current_tabstate().buffer.borrow().lines,
        &["abc", "defgh", "", "", "ijkl"]
    );
}
