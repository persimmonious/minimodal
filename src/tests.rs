use crate::app::{
    buffer::{Buffer, BufferPosition},
    editor::{actions::EditorAction, Editor, Mode},
    theme::Theme,
};
use std::{ffi::OsString, str::FromStr};

use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

fn type_lines(editor: &mut Editor, lines: &[&str]) {
    editor.execute_editor_action(EditorAction::EnterInsert);
    for (i, line) in lines.iter().enumerate() {
        for c in line.chars() {
            editor.execute_editor_action(EditorAction::InsertChar(c));
        }
        if i != lines.len() - 1 {
            editor.execute_editor_action(EditorAction::InsertLineBreak);
        }
    }
    editor.execute_editor_action(EditorAction::ExitInsert);
}

#[test]
fn test_create_unnamed_editor() {
    let editor = Editor::new(vec![Buffer::untitled()], Theme::default());
    assert_eq!(editor.current_bufpos(), BufferPosition { line: 0, col: 0 });
    assert_eq!(editor.get_mode(), &Mode::Normal);
    assert!(editor.current_buffer().read_name().is_none());
    assert!(editor.current_buffer().path().is_none());
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
    assert_eq!(editor.get_mode(), &Mode::Normal);
    assert_eq!(
        editor.current_buffer().read_name(),
        Some(OsString::from_str("newfile.txt").unwrap().as_os_str())
    );
    assert_eq!(
        editor.current_buffer().path(),
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
    editor.handle_key_press(KeyEvent::new(KeyCode::Char('i'), KeyModifiers::NONE));
    editor.handle_key_press(KeyEvent::new(KeyCode::Char('a'), KeyModifiers::NONE));
    editor.handle_key_press(KeyEvent::new(KeyCode::Char('b'), KeyModifiers::NONE));
    editor.handle_key_press(KeyEvent::new(KeyCode::Char('c'), KeyModifiers::NONE));
    editor.handle_key_press(KeyEvent::new(KeyCode::Char('d'), KeyModifiers::NONE));
    editor.handle_key_press(KeyEvent::new(KeyCode::Char('1'), KeyModifiers::NONE));
    editor.handle_key_press(KeyEvent::new(KeyCode::Char('2'), KeyModifiers::NONE));
    editor.handle_key_press(KeyEvent::new(KeyCode::Char('3'), KeyModifiers::NONE));
    editor.handle_key_press(KeyEvent::new(KeyCode::Char('!'), KeyModifiers::NONE));
    assert_eq!(editor.current_bufpos(), BufferPosition { line: 0, col: 8 });
    assert_eq!(editor.get_mode(), &Mode::Insert);
    editor.handle_key_press(KeyEvent::new(KeyCode::Esc, KeyModifiers::NONE));
    assert_eq!(editor.current_bufpos(), BufferPosition { line: 0, col: 7 });
    assert_eq!(editor.get_mode(), &Mode::Normal);
    assert_eq!(editor.current_buffer().lines, &["abcd123!".to_owned()]);
}

#[test]
fn test_multi_line_input() {
    let mut editor = Editor::new(vec![Buffer::untitled()], Theme::default());
    editor.handle_key_press(KeyEvent::new(KeyCode::Char('i'), KeyModifiers::NONE));
    assert_eq!(editor.current_bufpos(), BufferPosition { line: 0, col: 0 });
    editor.handle_key_press(KeyEvent::new(KeyCode::Enter, KeyModifiers::NONE));
    assert_eq!(editor.current_bufpos(), BufferPosition { line: 1, col: 0 });
    editor.handle_key_press(KeyEvent::new(KeyCode::Char('H'), KeyModifiers::NONE));
    editor.handle_key_press(KeyEvent::new(KeyCode::Char('e'), KeyModifiers::NONE));
    editor.handle_key_press(KeyEvent::new(KeyCode::Char('l'), KeyModifiers::NONE));
    editor.handle_key_press(KeyEvent::new(KeyCode::Char('l'), KeyModifiers::NONE));
    editor.handle_key_press(KeyEvent::new(KeyCode::Char('o'), KeyModifiers::NONE));
    assert_eq!(editor.current_bufpos(), BufferPosition { line: 1, col: 5 });
    editor.handle_key_press(KeyEvent::new(KeyCode::Enter, KeyModifiers::NONE));
    assert_eq!(editor.current_bufpos(), BufferPosition { line: 2, col: 0 });
    editor.handle_key_press(KeyEvent::new(KeyCode::Char('W'), KeyModifiers::NONE));
    editor.handle_key_press(KeyEvent::new(KeyCode::Char('o'), KeyModifiers::NONE));
    editor.handle_key_press(KeyEvent::new(KeyCode::Char('r'), KeyModifiers::NONE));
    editor.handle_key_press(KeyEvent::new(KeyCode::Char('l'), KeyModifiers::NONE));
    editor.handle_key_press(KeyEvent::new(KeyCode::Char('d'), KeyModifiers::NONE));
    assert_eq!(editor.current_bufpos(), BufferPosition { line: 2, col: 5 });
    editor.handle_key_press(KeyEvent::new(KeyCode::Enter, KeyModifiers::NONE));
    assert_eq!(editor.current_bufpos(), BufferPosition { line: 3, col: 0 });
    editor.handle_key_press(KeyEvent::new(KeyCode::Char('!'), KeyModifiers::NONE));
    assert_eq!(editor.current_bufpos(), BufferPosition { line: 3, col: 1 });
    editor.handle_key_press(KeyEvent::new(KeyCode::Esc, KeyModifiers::NONE));
    assert_eq!(editor.current_bufpos(), BufferPosition { line: 3, col: 0 });
    assert_eq!(editor.current_buffer().lines, &["", "Hello", "World", "!"]);
}

#[test]
fn test_text_with_multiple_line_breaks() {
    let mut editor = Editor::new(vec![Buffer::untitled()], Theme::default());
    editor.handle_key_press(KeyEvent::new(KeyCode::Char('i'), KeyModifiers::NONE));
    assert_eq!(editor.current_bufpos(), BufferPosition { line: 0, col: 0 });
    editor.handle_key_press(KeyEvent::new(KeyCode::Char('a'), KeyModifiers::NONE));
    editor.handle_key_press(KeyEvent::new(KeyCode::Char('b'), KeyModifiers::NONE));
    editor.handle_key_press(KeyEvent::new(KeyCode::Char('c'), KeyModifiers::NONE));
    assert_eq!(editor.current_bufpos(), BufferPosition { line: 0, col: 3 });
    editor.handle_key_press(KeyEvent::new(KeyCode::Enter, KeyModifiers::NONE));
    assert_eq!(editor.current_bufpos(), BufferPosition { line: 1, col: 0 });
    editor.handle_key_press(KeyEvent::new(KeyCode::Char('d'), KeyModifiers::NONE));
    editor.handle_key_press(KeyEvent::new(KeyCode::Char('e'), KeyModifiers::NONE));
    editor.handle_key_press(KeyEvent::new(KeyCode::Char('f'), KeyModifiers::NONE));
    editor.handle_key_press(KeyEvent::new(KeyCode::Char('g'), KeyModifiers::NONE));
    editor.handle_key_press(KeyEvent::new(KeyCode::Char('h'), KeyModifiers::NONE));
    assert_eq!(editor.current_bufpos(), BufferPosition { line: 1, col: 5 });
    editor.handle_key_press(KeyEvent::new(KeyCode::Enter, KeyModifiers::NONE));
    assert_eq!(editor.current_bufpos(), BufferPosition { line: 2, col: 0 });
    editor.handle_key_press(KeyEvent::new(KeyCode::Enter, KeyModifiers::NONE));
    assert_eq!(editor.current_bufpos(), BufferPosition { line: 3, col: 0 });
    editor.handle_key_press(KeyEvent::new(KeyCode::Enter, KeyModifiers::NONE));
    assert_eq!(editor.current_bufpos(), BufferPosition { line: 4, col: 0 });
    editor.handle_key_press(KeyEvent::new(KeyCode::Char('i'), KeyModifiers::NONE));
    editor.handle_key_press(KeyEvent::new(KeyCode::Char('j'), KeyModifiers::NONE));
    editor.handle_key_press(KeyEvent::new(KeyCode::Char('k'), KeyModifiers::NONE));
    editor.handle_key_press(KeyEvent::new(KeyCode::Char('l'), KeyModifiers::NONE));
    editor.handle_key_press(KeyEvent::new(KeyCode::Esc, KeyModifiers::NONE));
    assert_eq!(editor.current_bufpos(), BufferPosition { line: 4, col: 3 });
    assert_eq!(
        editor.current_buffer().lines,
        &["abc", "defgh", "", "", "ijkl"]
    );
}

#[test]
fn test_forward_deletion_in_empty_buffer_in_normal_mode() {
    let mut editor = Editor::new(vec![Buffer::untitled()], Theme::default());
    assert!(editor.current_buffer().lines.is_empty());
    assert_eq!(editor.current_bufpos(), BufferPosition { line: 0, col: 0 });
    editor.handle_key_press(KeyEvent::new(KeyCode::Char('x'), KeyModifiers::NONE));
    assert!(editor.current_buffer().lines.is_empty());
    assert_eq!(editor.current_bufpos(), BufferPosition { line: 0, col: 0 });
}

#[test]
fn test_forward_deletion_at_the_end_in_normal_mode() {
    let mut editor = Editor::new(vec![Buffer::untitled()], Theme::default());
    let lines = ["abcdefgh", "ijklmnop", "qrstu"];
    editor.handle_key_press(KeyEvent::new(KeyCode::Char('i'), KeyModifiers::NONE));
    for (i, line) in lines.iter().enumerate() {
        for c in line.chars() {
            editor.handle_key_press(KeyEvent::new(KeyCode::Char(c), KeyModifiers::NONE));
        }
        if i != lines.len() - 1 {
            editor.handle_key_press(KeyEvent::new(KeyCode::Enter, KeyModifiers::NONE));
        }
    }
    editor.handle_key_press(KeyEvent::new(KeyCode::Esc, KeyModifiers::NONE));
    assert_eq!(editor.current_buffer().lines, lines);
    assert_eq!(editor.current_bufpos(), BufferPosition { line: 2, col: 4 });
    editor.handle_key_press(KeyEvent::new(KeyCode::Char('x'), KeyModifiers::NONE));
    assert_eq!(
        editor.current_buffer().lines,
        ["abcdefgh", "ijklmnop", "qrst"]
    );
    assert_eq!(editor.current_bufpos(), BufferPosition { line: 2, col: 3 });
    editor.handle_key_press(KeyEvent::new(KeyCode::Char('x'), KeyModifiers::NONE));
    editor.handle_key_press(KeyEvent::new(KeyCode::Char('x'), KeyModifiers::NONE));
    editor.handle_key_press(KeyEvent::new(KeyCode::Char('x'), KeyModifiers::NONE));
    editor.handle_key_press(KeyEvent::new(KeyCode::Char('x'), KeyModifiers::NONE));
    assert_eq!(editor.current_buffer().lines, ["abcdefgh", "ijklmnop", ""]);
    assert_eq!(editor.current_bufpos(), BufferPosition { line: 2, col: 0 });
    editor.handle_key_press(KeyEvent::new(KeyCode::Char('x'), KeyModifiers::NONE));
    assert_eq!(editor.current_buffer().lines, ["abcdefgh", "ijklmnop", ""]);
    assert_eq!(editor.current_bufpos(), BufferPosition { line: 2, col: 0 });
}

#[test]
fn test_forward_deletion_in_the_middle_in_normal_mode() {
    let mut editor = Editor::new(vec![Buffer::untitled()], Theme::default());
    let lines = ["abcdefgh", "ijklmnop", "qrstu"];
    editor.handle_key_press(KeyEvent::new(KeyCode::Char('i'), KeyModifiers::NONE));
    for (i, line) in lines.iter().enumerate() {
        for c in line.chars() {
            editor.handle_key_press(KeyEvent::new(KeyCode::Char(c), KeyModifiers::NONE));
        }
        if i != lines.len() - 1 {
            editor.handle_key_press(KeyEvent::new(KeyCode::Enter, KeyModifiers::NONE));
        }
    }
    editor.handle_key_press(KeyEvent::new(KeyCode::Esc, KeyModifiers::NONE));
    assert_eq!(editor.current_buffer().lines, lines);
    assert_eq!(editor.current_bufpos(), BufferPosition { line: 2, col: 4 });
    editor.handle_key_press(KeyEvent::new(KeyCode::Char('h'), KeyModifiers::NONE));
    editor.handle_key_press(KeyEvent::new(KeyCode::Char('k'), KeyModifiers::NONE));
    assert_eq!(editor.current_bufpos(), BufferPosition { line: 1, col: 3 });
    editor.handle_key_press(KeyEvent::new(KeyCode::Char('x'), KeyModifiers::NONE));
    assert_eq!(
        editor.current_buffer().lines,
        ["abcdefgh", "ijkmnop", "qrstu"]
    );
    assert_eq!(editor.current_bufpos(), BufferPosition { line: 1, col: 3 });
    for _ in 0..4 {
        editor.handle_key_press(KeyEvent::new(KeyCode::Char('x'), KeyModifiers::NONE));
    }
    assert_eq!(editor.current_buffer().lines, ["abcdefgh", "ijk", "qrstu"]);
    assert_eq!(editor.current_bufpos(), BufferPosition { line: 1, col: 2 });
    for _ in 0..3 {
        editor.handle_key_press(KeyEvent::new(KeyCode::Char('x'), KeyModifiers::NONE));
    }
    assert_eq!(editor.current_buffer().lines, ["abcdefgh", "", "qrstu"]);
    assert_eq!(editor.current_bufpos(), BufferPosition { line: 1, col: 0 });
    editor.handle_key_press(KeyEvent::new(KeyCode::Char('x'), KeyModifiers::NONE));
    assert_eq!(editor.current_buffer().lines, ["abcdefgh", "", "qrstu"]);
    assert_eq!(editor.current_bufpos(), BufferPosition { line: 1, col: 0 });
}

#[test]
fn test_forward_deletion_at_the_start_in_normal_mode() {
    let mut editor = Editor::new(vec![Buffer::untitled()], Theme::default());
    let lines = ["abcdefgh", "ijklmnop", "qrstu"];
    editor.handle_key_press(KeyEvent::new(KeyCode::Char('i'), KeyModifiers::NONE));
    for (i, line) in lines.iter().enumerate() {
        for c in line.chars() {
            editor.handle_key_press(KeyEvent::new(KeyCode::Char(c), KeyModifiers::NONE));
        }
        if i != lines.len() - 1 {
            editor.handle_key_press(KeyEvent::new(KeyCode::Enter, KeyModifiers::NONE));
        }
    }
    editor.handle_key_press(KeyEvent::new(KeyCode::Esc, KeyModifiers::NONE));
    assert_eq!(editor.current_buffer().lines, lines);
    assert_eq!(editor.current_bufpos(), BufferPosition { line: 2, col: 4 });
    editor.handle_key_press(KeyEvent::new(KeyCode::Char('k'), KeyModifiers::NONE));
    editor.handle_key_press(KeyEvent::new(KeyCode::Char('0'), KeyModifiers::NONE));
    assert_eq!(editor.current_bufpos(), BufferPosition { line: 1, col: 0 });
    editor.handle_key_press(KeyEvent::new(KeyCode::Char('x'), KeyModifiers::NONE));
    assert_eq!(
        editor.current_buffer().lines,
        ["abcdefgh", "jklmnop", "qrstu"]
    );
    assert_eq!(editor.current_bufpos(), BufferPosition { line: 1, col: 0 });
    for _ in 0..3 {
        editor.handle_key_press(KeyEvent::new(KeyCode::Char('x'), KeyModifiers::NONE));
    }
    assert_eq!(editor.current_buffer().lines, ["abcdefgh", "mnop", "qrstu"]);
    assert_eq!(editor.current_bufpos(), BufferPosition { line: 1, col: 0 });
    for _ in 0..4 {
        editor.handle_key_press(KeyEvent::new(KeyCode::Char('x'), KeyModifiers::NONE));
    }
    assert_eq!(editor.current_buffer().lines, ["abcdefgh", "", "qrstu"]);
    assert_eq!(editor.current_bufpos(), BufferPosition { line: 1, col: 0 });
}

#[test]
fn test_backward_deletion_in_empty_buffer_in_normal_mode() {
    let mut editor = Editor::new(vec![Buffer::untitled()], Theme::default());
    assert!(editor.current_buffer().lines.is_empty());
    assert_eq!(editor.current_bufpos(), BufferPosition { line: 0, col: 0 });
    editor.handle_key_press(KeyEvent::new(KeyCode::Char('X'), KeyModifiers::NONE));
    assert!(editor.current_buffer().lines.is_empty());
    assert_eq!(editor.current_bufpos(), BufferPosition { line: 0, col: 0 });
}

#[test]
fn test_deletion_in_empty_buffer_in_insert_mode() {
    let mut editor = Editor::new(vec![Buffer::untitled()], Theme::default());
    assert!(editor.current_buffer().lines.is_empty());
    assert_eq!(editor.current_bufpos(), BufferPosition { line: 0, col: 0 });
    editor.handle_key_press(KeyEvent::new(KeyCode::Char('i'), KeyModifiers::NONE));
    editor.handle_key_press(KeyEvent::new(KeyCode::Backspace, KeyModifiers::NONE));
    assert!(editor.current_buffer().lines.is_empty());
    assert_eq!(editor.current_bufpos(), BufferPosition { line: 0, col: 0 });
    editor.handle_key_press(KeyEvent::new(KeyCode::Delete, KeyModifiers::NONE));
    assert!(editor.current_buffer().lines.is_empty());
    assert_eq!(editor.current_bufpos(), BufferPosition { line: 0, col: 0 });
}

#[test]
fn test_backward_deletion_at_the_start_in_normal_mode() {
    let mut editor = Editor::new(vec![Buffer::untitled()], Theme::default());
    let lines = ["abcdefgh", "ijklmnop", "qrstu"];
    editor.handle_key_press(KeyEvent::new(KeyCode::Char('i'), KeyModifiers::NONE));
    for (i, line) in lines.iter().enumerate() {
        for c in line.chars() {
            editor.handle_key_press(KeyEvent::new(KeyCode::Char(c), KeyModifiers::NONE));
        }
        if i != lines.len() - 1 {
            editor.handle_key_press(KeyEvent::new(KeyCode::Enter, KeyModifiers::NONE));
        }
    }
    editor.handle_key_press(KeyEvent::new(KeyCode::Esc, KeyModifiers::NONE));
    editor.handle_key_press(KeyEvent::new(KeyCode::Char('k'), KeyModifiers::NONE));
    editor.handle_key_press(KeyEvent::new(KeyCode::Char('k'), KeyModifiers::NONE));
    editor.handle_key_press(KeyEvent::new(KeyCode::Char('0'), KeyModifiers::NONE));
    assert_eq!(editor.current_buffer().lines, lines);
    assert_eq!(editor.current_bufpos(), BufferPosition { line: 0, col: 0 });
    editor.handle_key_press(KeyEvent::new(KeyCode::Char('X'), KeyModifiers::NONE));
    assert_eq!(editor.current_buffer().lines, lines);
    assert_eq!(editor.current_bufpos(), BufferPosition { line: 0, col: 0 });
}

#[test]
fn test_backward_deletion_middle_and_start_in_normal_mode() {
    let mut editor = Editor::new(vec![Buffer::untitled()], Theme::default());
    let lines = ["abcdefgh", "ijklmnop", "qrstu"];
    editor.handle_key_press(KeyEvent::new(KeyCode::Char('i'), KeyModifiers::NONE));
    for (i, line) in lines.iter().enumerate() {
        for c in line.chars() {
            editor.handle_key_press(KeyEvent::new(KeyCode::Char(c), KeyModifiers::NONE));
        }
        if i != lines.len() - 1 {
            editor.handle_key_press(KeyEvent::new(KeyCode::Enter, KeyModifiers::NONE));
        }
    }
    editor.handle_key_press(KeyEvent::new(KeyCode::Esc, KeyModifiers::NONE));
    editor.handle_key_press(KeyEvent::new(KeyCode::Char('k'), KeyModifiers::NONE));
    assert_eq!(editor.current_buffer().lines, lines);
    assert_eq!(editor.current_bufpos(), BufferPosition { line: 1, col: 4 });
    editor.handle_key_press(KeyEvent::new(KeyCode::Char('X'), KeyModifiers::NONE));
    assert_eq!(
        editor.current_buffer().lines,
        ["abcdefgh", "ijkmnop", "qrstu"]
    );
    assert_eq!(editor.current_bufpos(), BufferPosition { line: 1, col: 3 });
    editor.handle_key_press(KeyEvent::new(KeyCode::Char('X'), KeyModifiers::NONE));
    editor.handle_key_press(KeyEvent::new(KeyCode::Char('X'), KeyModifiers::NONE));
    editor.handle_key_press(KeyEvent::new(KeyCode::Char('X'), KeyModifiers::NONE));
    assert_eq!(editor.current_buffer().lines, ["abcdefgh", "mnop", "qrstu"]);
    assert_eq!(editor.current_bufpos(), BufferPosition { line: 1, col: 0 });
    editor.handle_key_press(KeyEvent::new(KeyCode::Char('X'), KeyModifiers::NONE));
    assert_eq!(editor.current_buffer().lines, ["abcdefgh", "mnop", "qrstu"]);
    assert_eq!(editor.current_bufpos(), BufferPosition { line: 1, col: 0 });
}

#[test]
fn test_backward_deletion_in_empty_line_in_normal_mode() {
    let mut editor = Editor::new(vec![Buffer::untitled()], Theme::default());
    let lines = ["abcdefgh", "", "qrstu"];
    editor.handle_key_press(KeyEvent::new(KeyCode::Char('i'), KeyModifiers::NONE));
    for (i, line) in lines.iter().enumerate() {
        for c in line.chars() {
            editor.handle_key_press(KeyEvent::new(KeyCode::Char(c), KeyModifiers::NONE));
        }
        if i != lines.len() - 1 {
            editor.handle_key_press(KeyEvent::new(KeyCode::Enter, KeyModifiers::NONE));
        }
    }
    editor.handle_key_press(KeyEvent::new(KeyCode::Esc, KeyModifiers::NONE));
    editor.handle_key_press(KeyEvent::new(KeyCode::Char('k'), KeyModifiers::NONE));
    assert_eq!(editor.current_buffer().lines, lines);
    assert_eq!(editor.current_bufpos(), BufferPosition { line: 1, col: 0 });
    editor.handle_key_press(KeyEvent::new(KeyCode::Char('X'), KeyModifiers::NONE));
    assert_eq!(editor.current_buffer().lines, lines);
    assert_eq!(editor.current_bufpos(), BufferPosition { line: 1, col: 0 });
}

#[test]
fn test_backward_deletion_at_the_end_in_normal_mode() {
    let mut editor = Editor::new(vec![Buffer::untitled()], Theme::default());
    let lines = ["abcdefgh", "ijklmnop", "qrstu"];
    editor.handle_key_press(KeyEvent::new(KeyCode::Char('i'), KeyModifiers::NONE));
    for (i, line) in lines.iter().enumerate() {
        for c in line.chars() {
            editor.handle_key_press(KeyEvent::new(KeyCode::Char(c), KeyModifiers::NONE));
        }
        if i != lines.len() - 1 {
            editor.handle_key_press(KeyEvent::new(KeyCode::Enter, KeyModifiers::NONE));
        }
    }
    editor.handle_key_press(KeyEvent::new(KeyCode::Esc, KeyModifiers::NONE));
    assert_eq!(editor.current_buffer().lines, lines);
    assert_eq!(editor.current_bufpos(), BufferPosition { line: 2, col: 4 });
    editor.handle_key_press(KeyEvent::new(KeyCode::Char('X'), KeyModifiers::NONE));
    assert_eq!(
        editor.current_buffer().lines,
        ["abcdefgh", "ijklmnop", "qrsu"]
    );
    assert_eq!(editor.current_bufpos(), BufferPosition { line: 2, col: 3 });
    editor.handle_key_press(KeyEvent::new(KeyCode::Char('X'), KeyModifiers::NONE));
    editor.handle_key_press(KeyEvent::new(KeyCode::Char('X'), KeyModifiers::NONE));
    editor.handle_key_press(KeyEvent::new(KeyCode::Char('X'), KeyModifiers::NONE));
    assert_eq!(editor.current_buffer().lines, ["abcdefgh", "ijklmnop", "u"]);
    assert_eq!(editor.current_bufpos(), BufferPosition { line: 2, col: 0 });
    editor.handle_key_press(KeyEvent::new(KeyCode::Char('X'), KeyModifiers::NONE));
    assert_eq!(editor.current_buffer().lines, ["abcdefgh", "ijklmnop", "u"]);
    assert_eq!(editor.current_bufpos(), BufferPosition { line: 2, col: 0 });
}

#[test]
fn test_forward_deletion_at_the_end_in_insert_mode() {
    let mut editor = Editor::new(vec![Buffer::untitled()], Theme::default());
    let lines = ["abcdefgh", "ijklmnop", "qrstu"];
    editor.handle_key_press(KeyEvent::new(KeyCode::Char('i'), KeyModifiers::NONE));
    for (i, line) in lines.iter().enumerate() {
        for c in line.chars() {
            editor.handle_key_press(KeyEvent::new(KeyCode::Char(c), KeyModifiers::NONE));
        }
        if i != lines.len() - 1 {
            editor.handle_key_press(KeyEvent::new(KeyCode::Enter, KeyModifiers::NONE));
        }
    }
    assert_eq!(editor.current_buffer().lines, lines);
    assert_eq!(editor.current_bufpos(), BufferPosition { line: 2, col: 5 });
    editor.handle_key_press(KeyEvent::new(KeyCode::Delete, KeyModifiers::NONE));
    assert_eq!(editor.current_buffer().lines, lines);
    assert_eq!(editor.current_bufpos(), BufferPosition { line: 2, col: 5 });
}

#[test]
fn test_forward_deletion_middle_and_end_in_insert_mode() {
    let mut editor = Editor::new(vec![Buffer::untitled()], Theme::default());
    let lines = ["abcdefgh", "ijklmnop", "qrstu"];
    editor.handle_key_press(KeyEvent::new(KeyCode::Char('i'), KeyModifiers::NONE));
    for (i, line) in lines.iter().enumerate() {
        for c in line.chars() {
            editor.handle_key_press(KeyEvent::new(KeyCode::Char(c), KeyModifiers::NONE));
        }
        if i != lines.len() - 1 {
            editor.handle_key_press(KeyEvent::new(KeyCode::Enter, KeyModifiers::NONE));
        }
    }
    editor.handle_key_press(KeyEvent::new(KeyCode::Esc, KeyModifiers::NONE));
    editor.handle_key_press(KeyEvent::new(KeyCode::Char('k'), KeyModifiers::NONE));
    editor.handle_key_press(KeyEvent::new(KeyCode::Char('i'), KeyModifiers::NONE));
    assert_eq!(editor.current_buffer().lines, lines);
    assert_eq!(editor.current_bufpos(), BufferPosition { line: 1, col: 4 });
    editor.handle_key_press(KeyEvent::new(KeyCode::Delete, KeyModifiers::NONE));
    assert_eq!(
        editor.current_buffer().lines,
        ["abcdefgh", "ijklnop", "qrstu"]
    );
    assert_eq!(editor.current_bufpos(), BufferPosition { line: 1, col: 4 });
    editor.handle_key_press(KeyEvent::new(KeyCode::Delete, KeyModifiers::NONE));
    editor.handle_key_press(KeyEvent::new(KeyCode::Delete, KeyModifiers::NONE));
    editor.handle_key_press(KeyEvent::new(KeyCode::Delete, KeyModifiers::NONE));
    assert_eq!(editor.current_buffer().lines, ["abcdefgh", "ijkl", "qrstu"]);
    assert_eq!(editor.current_bufpos(), BufferPosition { line: 1, col: 4 });
    editor.handle_key_press(KeyEvent::new(KeyCode::Delete, KeyModifiers::NONE));
    assert_eq!(editor.current_buffer().lines, ["abcdefgh", "ijklqrstu"]);
    assert_eq!(editor.current_bufpos(), BufferPosition { line: 1, col: 4 });
}

#[test]
fn test_forward_deletion_at_the_start_in_insert_mode() {
    let mut editor = Editor::new(vec![Buffer::untitled()], Theme::default());
    let lines = ["abcdefgh", "ijklmnop", "qrstu"];
    editor.handle_key_press(KeyEvent::new(KeyCode::Char('i'), KeyModifiers::NONE));
    for (i, line) in lines.iter().enumerate() {
        for c in line.chars() {
            editor.handle_key_press(KeyEvent::new(KeyCode::Char(c), KeyModifiers::NONE));
        }
        if i != lines.len() - 1 {
            editor.handle_key_press(KeyEvent::new(KeyCode::Enter, KeyModifiers::NONE));
        }
    }
    editor.handle_key_press(KeyEvent::new(KeyCode::Esc, KeyModifiers::NONE));
    editor.handle_key_press(KeyEvent::new(KeyCode::Char('k'), KeyModifiers::NONE));
    editor.handle_key_press(KeyEvent::new(KeyCode::Char('k'), KeyModifiers::NONE));
    editor.handle_key_press(KeyEvent::new(KeyCode::Char('0'), KeyModifiers::NONE));
    editor.handle_key_press(KeyEvent::new(KeyCode::Char('i'), KeyModifiers::NONE));
    assert_eq!(editor.current_buffer().lines, lines);
    assert_eq!(editor.current_bufpos(), BufferPosition { line: 0, col: 0 });
    editor.handle_key_press(KeyEvent::new(KeyCode::Delete, KeyModifiers::NONE));
    assert_eq!(
        editor.current_buffer().lines,
        ["bcdefgh", "ijklmnop", "qrstu"]
    );
    assert_eq!(editor.current_bufpos(), BufferPosition { line: 0, col: 0 });
}

#[test]
fn test_forward_deletion_appending_empty_line() {
    let mut editor = Editor::new(vec![Buffer::untitled()], Theme::default());
    let lines = ["abcdefgh", "ijklmnop", "", "", "qrstu"];
    editor.handle_key_press(KeyEvent::new(KeyCode::Char('i'), KeyModifiers::NONE));
    for (i, line) in lines.iter().enumerate() {
        for c in line.chars() {
            editor.handle_key_press(KeyEvent::new(KeyCode::Char(c), KeyModifiers::NONE));
        }
        if i != lines.len() - 1 {
            editor.handle_key_press(KeyEvent::new(KeyCode::Enter, KeyModifiers::NONE));
        }
    }
    editor.handle_key_press(KeyEvent::new(KeyCode::Esc, KeyModifiers::NONE));
    editor.handle_key_press(KeyEvent::new(KeyCode::Char('k'), KeyModifiers::NONE));
    editor.handle_key_press(KeyEvent::new(KeyCode::Char('k'), KeyModifiers::NONE));
    editor.handle_key_press(KeyEvent::new(KeyCode::Char('k'), KeyModifiers::NONE));
    editor.handle_key_press(KeyEvent::new(KeyCode::Char('A'), KeyModifiers::NONE));
    assert_eq!(editor.current_buffer().lines, lines);
    assert_eq!(editor.current_bufpos(), BufferPosition { line: 1, col: 8 });
    editor.handle_key_press(KeyEvent::new(KeyCode::Delete, KeyModifiers::NONE));
    assert_eq!(
        editor.current_buffer().lines,
        ["abcdefgh", "ijklmnop", "", "qrstu"]
    );
    assert_eq!(editor.current_bufpos(), BufferPosition { line: 1, col: 8 });
}

#[test]
fn test_forward_deletion_appending_to_empty_line() {
    let mut editor = Editor::new(vec![Buffer::untitled()], Theme::default());
    let lines = ["abcdefgh", "ijklmnop", "", "", "qrstu"];
    editor.handle_key_press(KeyEvent::new(KeyCode::Char('i'), KeyModifiers::NONE));
    for (i, line) in lines.iter().enumerate() {
        for c in line.chars() {
            editor.handle_key_press(KeyEvent::new(KeyCode::Char(c), KeyModifiers::NONE));
        }
        if i != lines.len() - 1 {
            editor.handle_key_press(KeyEvent::new(KeyCode::Enter, KeyModifiers::NONE));
        }
    }
    editor.handle_key_press(KeyEvent::new(KeyCode::Esc, KeyModifiers::NONE));
    editor.handle_key_press(KeyEvent::new(KeyCode::Char('k'), KeyModifiers::NONE));
    editor.handle_key_press(KeyEvent::new(KeyCode::Char('A'), KeyModifiers::NONE));
    assert_eq!(editor.current_buffer().lines, lines);
    assert_eq!(editor.current_bufpos(), BufferPosition { line: 3, col: 0 });
    editor.handle_key_press(KeyEvent::new(KeyCode::Delete, KeyModifiers::NONE));
    assert_eq!(
        editor.current_buffer().lines,
        ["abcdefgh", "ijklmnop", "", "qrstu"]
    );
    assert_eq!(editor.current_bufpos(), BufferPosition { line: 3, col: 0 });
}

#[test]
fn test_forward_deletion_joining_empty_lines() {
    let mut editor = Editor::new(vec![Buffer::untitled()], Theme::default());
    let lines = ["abcdefgh", "ijklmnop", "", "", "qrstu"];
    editor.handle_key_press(KeyEvent::new(KeyCode::Char('i'), KeyModifiers::NONE));
    for (i, line) in lines.iter().enumerate() {
        for c in line.chars() {
            editor.handle_key_press(KeyEvent::new(KeyCode::Char(c), KeyModifiers::NONE));
        }
        if i != lines.len() - 1 {
            editor.handle_key_press(KeyEvent::new(KeyCode::Enter, KeyModifiers::NONE));
        }
    }
    editor.handle_key_press(KeyEvent::new(KeyCode::Esc, KeyModifiers::NONE));
    editor.handle_key_press(KeyEvent::new(KeyCode::Char('k'), KeyModifiers::NONE));
    editor.handle_key_press(KeyEvent::new(KeyCode::Char('k'), KeyModifiers::NONE));
    editor.handle_key_press(KeyEvent::new(KeyCode::Char('A'), KeyModifiers::NONE));
    assert_eq!(editor.current_buffer().lines, lines);
    assert_eq!(editor.current_bufpos(), BufferPosition { line: 2, col: 0 });
    editor.handle_key_press(KeyEvent::new(KeyCode::Delete, KeyModifiers::NONE));
    assert_eq!(
        editor.current_buffer().lines,
        ["abcdefgh", "ijklmnop", "", "qrstu"]
    );
    assert_eq!(editor.current_bufpos(), BufferPosition { line: 2, col: 0 });
}

#[test]
fn test_backward_deletion_at_the_end_in_insert_mode() {
    let mut editor = Editor::new(vec![Buffer::untitled()], Theme::default());
    let lines = ["abcdefgh", "ijklmnop", "qrstu"];
    editor.handle_key_press(KeyEvent::new(KeyCode::Char('i'), KeyModifiers::NONE));
    for (i, line) in lines.iter().enumerate() {
        for c in line.chars() {
            editor.handle_key_press(KeyEvent::new(KeyCode::Char(c), KeyModifiers::NONE));
        }
        if i != lines.len() - 1 {
            editor.handle_key_press(KeyEvent::new(KeyCode::Enter, KeyModifiers::NONE));
        }
    }
    assert_eq!(editor.current_buffer().lines, lines);
    assert_eq!(editor.current_bufpos(), BufferPosition { line: 2, col: 5 });
    editor.handle_key_press(KeyEvent::new(KeyCode::Backspace, KeyModifiers::NONE));
    assert_eq!(
        editor.current_buffer().lines,
        ["abcdefgh", "ijklmnop", "qrst"]
    );
    assert_eq!(editor.current_bufpos(), BufferPosition { line: 2, col: 4 });
    for _ in 0..4 {
        editor.handle_key_press(KeyEvent::new(KeyCode::Backspace, KeyModifiers::NONE));
    }
    assert_eq!(editor.current_buffer().lines, ["abcdefgh", "ijklmnop", ""]);
    assert_eq!(editor.current_bufpos(), BufferPosition { line: 2, col: 0 });
    editor.handle_key_press(KeyEvent::new(KeyCode::Backspace, KeyModifiers::NONE));
    assert_eq!(editor.current_buffer().lines, ["abcdefgh", "ijklmnop"]);
    assert_eq!(editor.current_bufpos(), BufferPosition { line: 1, col: 8 });
}

#[test]
fn test_backward_deletion_in_the_middle_in_insert_mode() {
    let mut editor = Editor::new(vec![Buffer::untitled()], Theme::default());
    let lines = ["abcdefgh", "ijklmnop", "qrstu"];
    editor.handle_key_press(KeyEvent::new(KeyCode::Char('i'), KeyModifiers::NONE));
    for (i, line) in lines.iter().enumerate() {
        for c in line.chars() {
            editor.handle_key_press(KeyEvent::new(KeyCode::Char(c), KeyModifiers::NONE));
        }
        if i != lines.len() - 1 {
            editor.handle_key_press(KeyEvent::new(KeyCode::Enter, KeyModifiers::NONE));
        }
    }
    editor.handle_key_press(KeyEvent::new(KeyCode::Esc, KeyModifiers::NONE));
    editor.handle_key_press(KeyEvent::new(KeyCode::Char('k'), KeyModifiers::NONE));
    assert_eq!(editor.current_buffer().lines, lines);
    assert_eq!(editor.current_bufpos(), BufferPosition { line: 1, col: 4 });
    editor.handle_key_press(KeyEvent::new(KeyCode::Char('i'), KeyModifiers::NONE));
    editor.handle_key_press(KeyEvent::new(KeyCode::Backspace, KeyModifiers::NONE));
    assert_eq!(
        editor.current_buffer().lines,
        ["abcdefgh", "ijkmnop", "qrstu"]
    );
    assert_eq!(editor.current_bufpos(), BufferPosition { line: 1, col: 3 });
    editor.handle_key_press(KeyEvent::new(KeyCode::Backspace, KeyModifiers::NONE));
    editor.handle_key_press(KeyEvent::new(KeyCode::Backspace, KeyModifiers::NONE));
    editor.handle_key_press(KeyEvent::new(KeyCode::Backspace, KeyModifiers::NONE));
    assert_eq!(editor.current_buffer().lines, ["abcdefgh", "mnop", "qrstu"]);
    assert_eq!(editor.current_bufpos(), BufferPosition { line: 1, col: 0 });
    editor.handle_key_press(KeyEvent::new(KeyCode::Backspace, KeyModifiers::NONE));
    assert_eq!(editor.current_buffer().lines, ["abcdefghmnop", "qrstu"]);
    assert_eq!(editor.current_bufpos(), BufferPosition { line: 0, col: 8 });
}

#[test]
fn test_backward_deletion_at_the_start_in_insert_mode() {
    let mut editor = Editor::new(vec![Buffer::untitled()], Theme::default());
    let lines = ["abcdefgh", "ijklmnop", "qrstu"];
    editor.handle_key_press(KeyEvent::new(KeyCode::Char('i'), KeyModifiers::NONE));
    for (i, line) in lines.iter().enumerate() {
        for c in line.chars() {
            editor.handle_key_press(KeyEvent::new(KeyCode::Char(c), KeyModifiers::NONE));
        }
        if i != lines.len() - 1 {
            editor.handle_key_press(KeyEvent::new(KeyCode::Enter, KeyModifiers::NONE));
        }
    }
    editor.handle_key_press(KeyEvent::new(KeyCode::Esc, KeyModifiers::NONE));
    editor.handle_key_press(KeyEvent::new(KeyCode::Char('k'), KeyModifiers::NONE));
    editor.handle_key_press(KeyEvent::new(KeyCode::Char('k'), KeyModifiers::NONE));
    editor.handle_key_press(KeyEvent::new(KeyCode::Char('I'), KeyModifiers::NONE));
    assert_eq!(editor.current_buffer().lines, lines);
    assert_eq!(editor.current_bufpos(), BufferPosition { line: 0, col: 0 });
    editor.handle_key_press(KeyEvent::new(KeyCode::Backspace, KeyModifiers::NONE));
    assert_eq!(editor.current_buffer().lines, lines);
    assert_eq!(editor.current_bufpos(), BufferPosition { line: 0, col: 0 });
}

#[test]
fn test_backward_deletion_appending_to_empty_line() {
    let mut editor = Editor::new(vec![Buffer::untitled()], Theme::default());
    let lines = ["abcdefgh", "ijklmnop", "", "", "qrstu"];
    editor.handle_key_press(KeyEvent::new(KeyCode::Char('i'), KeyModifiers::NONE));
    for (i, line) in lines.iter().enumerate() {
        for c in line.chars() {
            editor.handle_key_press(KeyEvent::new(KeyCode::Char(c), KeyModifiers::NONE));
        }
        if i != lines.len() - 1 {
            editor.handle_key_press(KeyEvent::new(KeyCode::Enter, KeyModifiers::NONE));
        }
    }
    editor.handle_key_press(KeyEvent::new(KeyCode::Esc, KeyModifiers::NONE));
    editor.handle_key_press(KeyEvent::new(KeyCode::Char('I'), KeyModifiers::NONE));
    assert_eq!(editor.current_buffer().lines, lines);
    assert_eq!(editor.current_bufpos(), BufferPosition { line: 4, col: 0 });
    editor.handle_key_press(KeyEvent::new(KeyCode::Backspace, KeyModifiers::NONE));
    assert_eq!(
        editor.current_buffer().lines,
        ["abcdefgh", "ijklmnop", "", "qrstu"]
    );
    assert_eq!(editor.current_bufpos(), BufferPosition { line: 3, col: 0 });
}

#[test]
fn test_backward_deletion_joining_empty_lines() {
    let mut editor = Editor::new(vec![Buffer::untitled()], Theme::default());
    let lines = ["abcdefgh", "ijklmnop", "", "", "qrstu"];
    editor.handle_key_press(KeyEvent::new(KeyCode::Char('i'), KeyModifiers::NONE));
    for (i, line) in lines.iter().enumerate() {
        for c in line.chars() {
            editor.handle_key_press(KeyEvent::new(KeyCode::Char(c), KeyModifiers::NONE));
        }
        if i != lines.len() - 1 {
            editor.handle_key_press(KeyEvent::new(KeyCode::Enter, KeyModifiers::NONE));
        }
    }
    editor.handle_key_press(KeyEvent::new(KeyCode::Esc, KeyModifiers::NONE));
    editor.handle_key_press(KeyEvent::new(KeyCode::Char('k'), KeyModifiers::NONE));
    editor.handle_key_press(KeyEvent::new(KeyCode::Char('i'), KeyModifiers::NONE));
    assert_eq!(editor.current_buffer().lines, lines);
    assert_eq!(editor.current_bufpos(), BufferPosition { line: 3, col: 0 });
    editor.handle_key_press(KeyEvent::new(KeyCode::Backspace, KeyModifiers::NONE));
    assert_eq!(
        editor.current_buffer().lines,
        ["abcdefgh", "ijklmnop", "", "qrstu"]
    );
    assert_eq!(editor.current_bufpos(), BufferPosition { line: 2, col: 0 });
}

#[test]
fn test_normal_mode_arrow_movement() {
    let mut editor = Editor::new(vec![Buffer::untitled()], Theme::default());
    let lines = ["abcdefg", "hij", "klmnop", "", "qrstu"];
    type_lines(&mut editor, &lines);
    assert_eq!(editor.current_buffer().lines, lines);
    assert_eq!(editor.current_bufpos(), BufferPosition { line: 4, col: 4 });
    editor.handle_key_press(KeyEvent::new(KeyCode::Down, KeyModifiers::NONE));
    assert_eq!(editor.current_bufpos(), BufferPosition { line: 4, col: 4 });
    editor.handle_key_press(KeyEvent::new(KeyCode::Up, KeyModifiers::NONE));
    assert_eq!(editor.current_bufpos(), BufferPosition { line: 3, col: 0 });
    editor.handle_key_press(KeyEvent::new(KeyCode::Up, KeyModifiers::NONE));
    assert_eq!(editor.current_bufpos(), BufferPosition { line: 2, col: 4 });
    editor.handle_key_press(KeyEvent::new(KeyCode::Up, KeyModifiers::NONE));
    assert_eq!(editor.current_bufpos(), BufferPosition { line: 1, col: 2 });
    editor.handle_key_press(KeyEvent::new(KeyCode::Up, KeyModifiers::NONE));
    assert_eq!(editor.current_bufpos(), BufferPosition { line: 0, col: 4 });
    editor.handle_key_press(KeyEvent::new(KeyCode::Up, KeyModifiers::NONE));
    assert_eq!(editor.current_bufpos(), BufferPosition { line: 0, col: 4 });
    editor.handle_key_press(KeyEvent::new(KeyCode::Right, KeyModifiers::NONE));
    assert_eq!(editor.current_bufpos(), BufferPosition { line: 0, col: 5 });
    editor.handle_key_press(KeyEvent::new(KeyCode::Right, KeyModifiers::NONE));
    assert_eq!(editor.current_bufpos(), BufferPosition { line: 0, col: 6 });
    editor.handle_key_press(KeyEvent::new(KeyCode::Right, KeyModifiers::NONE));
    assert_eq!(editor.current_bufpos(), BufferPosition { line: 0, col: 6 });
    editor.handle_key_press(KeyEvent::new(KeyCode::Left, KeyModifiers::NONE));
    assert_eq!(editor.current_bufpos(), BufferPosition { line: 0, col: 5 });
    for _ in 0..5 {
        editor.handle_key_press(KeyEvent::new(KeyCode::Left, KeyModifiers::NONE));
    }
    assert_eq!(editor.current_bufpos(), BufferPosition { line: 0, col: 0 });
    editor.handle_key_press(KeyEvent::new(KeyCode::Left, KeyModifiers::NONE));
    assert_eq!(editor.current_bufpos(), BufferPosition { line: 0, col: 0 });
    editor.handle_key_press(KeyEvent::new(KeyCode::Right, KeyModifiers::NONE));
    assert_eq!(editor.current_bufpos(), BufferPosition { line: 0, col: 1 });
    editor.handle_key_press(KeyEvent::new(KeyCode::Right, KeyModifiers::NONE));
    assert_eq!(editor.current_bufpos(), BufferPosition { line: 0, col: 2 });
    editor.handle_key_press(KeyEvent::new(KeyCode::Down, KeyModifiers::NONE));
    assert_eq!(editor.current_bufpos(), BufferPosition { line: 1, col: 2 });
    editor.handle_key_press(KeyEvent::new(KeyCode::Down, KeyModifiers::NONE));
    assert_eq!(editor.current_bufpos(), BufferPosition { line: 2, col: 2 });
    editor.handle_key_press(KeyEvent::new(KeyCode::Down, KeyModifiers::NONE));
    assert_eq!(editor.current_bufpos(), BufferPosition { line: 3, col: 0 });
    editor.handle_key_press(KeyEvent::new(KeyCode::Left, KeyModifiers::NONE));
    assert_eq!(editor.current_bufpos(), BufferPosition { line: 3, col: 0 });
    editor.handle_key_press(KeyEvent::new(KeyCode::Right, KeyModifiers::NONE));
    assert_eq!(editor.current_bufpos(), BufferPosition { line: 3, col: 0 });
    editor.handle_key_press(KeyEvent::new(KeyCode::Down, KeyModifiers::NONE));
    assert_eq!(editor.current_bufpos(), BufferPosition { line: 4, col: 2 });
    assert_eq!(editor.current_buffer().lines, lines);
}

#[test]
fn test_empty_buffer_normal_mode() {
    let mut editor = Editor::new(vec![Buffer::untitled()], Theme::default());
    assert!(editor.current_buffer().lines.is_empty());
    assert_eq!(editor.current_bufpos(), BufferPosition { line: 0, col: 0 });
    editor.handle_key_press(KeyEvent::new(KeyCode::Down, KeyModifiers::NONE));
    assert_eq!(editor.current_bufpos(), BufferPosition { line: 0, col: 0 });
    editor.handle_key_press(KeyEvent::new(KeyCode::Up, KeyModifiers::NONE));
    assert_eq!(editor.current_bufpos(), BufferPosition { line: 0, col: 0 });
    editor.handle_key_press(KeyEvent::new(KeyCode::Left, KeyModifiers::NONE));
    assert_eq!(editor.current_bufpos(), BufferPosition { line: 0, col: 0 });
    editor.handle_key_press(KeyEvent::new(KeyCode::Right, KeyModifiers::NONE));
    assert_eq!(editor.current_bufpos(), BufferPosition { line: 0, col: 0 });
}

#[test]
fn test_empty_buffer_insert_mode() {
    let mut editor = Editor::new(vec![Buffer::untitled()], Theme::default());
    assert!(editor.current_buffer().lines.is_empty());
    assert_eq!(editor.current_bufpos(), BufferPosition { line: 0, col: 0 });
    editor.handle_key_press(KeyEvent::new(KeyCode::Char('i'), KeyModifiers::NONE));
    assert_eq!(editor.current_bufpos(), BufferPosition { line: 0, col: 0 });
    editor.handle_key_press(KeyEvent::new(KeyCode::Down, KeyModifiers::NONE));
    assert_eq!(editor.current_bufpos(), BufferPosition { line: 0, col: 0 });
    editor.handle_key_press(KeyEvent::new(KeyCode::Up, KeyModifiers::NONE));
    assert_eq!(editor.current_bufpos(), BufferPosition { line: 0, col: 0 });
    editor.handle_key_press(KeyEvent::new(KeyCode::Left, KeyModifiers::NONE));
    assert_eq!(editor.current_bufpos(), BufferPosition { line: 0, col: 0 });
    editor.handle_key_press(KeyEvent::new(KeyCode::Right, KeyModifiers::NONE));
    assert_eq!(editor.current_bufpos(), BufferPosition { line: 0, col: 0 });
    editor.handle_key_press(KeyEvent::new(KeyCode::Home, KeyModifiers::NONE));
    assert_eq!(editor.current_bufpos(), BufferPosition { line: 0, col: 0 });
    editor.handle_key_press(KeyEvent::new(KeyCode::End, KeyModifiers::NONE));
    assert_eq!(editor.current_bufpos(), BufferPosition { line: 0, col: 0 });
    editor.handle_key_press(KeyEvent::new(KeyCode::Esc, KeyModifiers::NONE));
    assert_eq!(editor.current_bufpos(), BufferPosition { line: 0, col: 0 });
}

#[test]
fn test_horizontal_movement_insert_mode() {
    let mut editor = Editor::new(vec![Buffer::untitled()], Theme::default());
    let lines = ["abcdefg", "hij", "klmnop", "", "qrstu"];
    type_lines(&mut editor, &lines);
    assert_eq!(editor.current_buffer().lines, lines);
    assert_eq!(editor.current_bufpos(), BufferPosition { line: 4, col: 4 });
    editor.handle_key_press(KeyEvent::new(KeyCode::Char('a'), KeyModifiers::NONE));
    assert_eq!(editor.current_bufpos(), BufferPosition { line: 4, col: 5 });
    editor.handle_key_press(KeyEvent::new(KeyCode::Right, KeyModifiers::NONE));
    assert_eq!(editor.current_bufpos(), BufferPosition { line: 4, col: 5 });
    editor.handle_key_press(KeyEvent::new(KeyCode::Left, KeyModifiers::NONE));
    assert_eq!(editor.current_bufpos(), BufferPosition { line: 4, col: 4 });
    for _ in 0..4 {
        editor.handle_key_press(KeyEvent::new(KeyCode::Left, KeyModifiers::NONE));
    }
    assert_eq!(editor.current_bufpos(), BufferPosition { line: 4, col: 0 });
    editor.handle_key_press(KeyEvent::new(KeyCode::Left, KeyModifiers::NONE));
    assert_eq!(editor.current_bufpos(), BufferPosition { line: 3, col: 0 });
    editor.handle_key_press(KeyEvent::new(KeyCode::Left, KeyModifiers::NONE));
    assert_eq!(editor.current_bufpos(), BufferPosition { line: 2, col: 6 });
    for _ in 0..6 {
        editor.handle_key_press(KeyEvent::new(KeyCode::Left, KeyModifiers::NONE));
    }
    assert_eq!(editor.current_bufpos(), BufferPosition { line: 2, col: 0 });
    for _ in 0..4 {
        editor.handle_key_press(KeyEvent::new(KeyCode::Left, KeyModifiers::NONE));
    }
    assert_eq!(editor.current_bufpos(), BufferPosition { line: 1, col: 0 });
    for _ in 0..8 {
        editor.handle_key_press(KeyEvent::new(KeyCode::Left, KeyModifiers::NONE));
    }
    assert_eq!(editor.current_bufpos(), BufferPosition { line: 0, col: 0 });
    editor.handle_key_press(KeyEvent::new(KeyCode::Left, KeyModifiers::NONE));
    assert_eq!(editor.current_bufpos(), BufferPosition { line: 0, col: 0 });
    for _ in 0..7 {
        editor.handle_key_press(KeyEvent::new(KeyCode::Right, KeyModifiers::NONE));
    }
    assert_eq!(editor.current_bufpos(), BufferPosition { line: 0, col: 7 });
    editor.handle_key_press(KeyEvent::new(KeyCode::Right, KeyModifiers::NONE));
    assert_eq!(editor.current_bufpos(), BufferPosition { line: 1, col: 0 });
    for _ in 0..10 {
        editor.handle_key_press(KeyEvent::new(KeyCode::Right, KeyModifiers::NONE));
    }
    assert_eq!(editor.current_bufpos(), BufferPosition { line: 2, col: 6 });
    editor.handle_key_press(KeyEvent::new(KeyCode::Right, KeyModifiers::NONE));
    assert_eq!(editor.current_bufpos(), BufferPosition { line: 3, col: 0 });
    editor.handle_key_press(KeyEvent::new(KeyCode::Right, KeyModifiers::NONE));
    assert_eq!(editor.current_bufpos(), BufferPosition { line: 4, col: 0 });
    for _ in 0..5 {
        editor.handle_key_press(KeyEvent::new(KeyCode::Right, KeyModifiers::NONE));
    }
    assert_eq!(editor.current_bufpos(), BufferPosition { line: 4, col: 5 });
}

#[test]
fn test_vertical_movement_insert_mode() {
    let mut editor = Editor::new(vec![Buffer::untitled()], Theme::default());
    let lines = ["abcdefg", "hij", "klmnop", "", "qrstu"];
    type_lines(&mut editor, &lines);
    assert_eq!(editor.current_bufpos(), BufferPosition { line: 4, col: 4 });
    editor.handle_key_press(KeyEvent::new(KeyCode::Char('i'), KeyModifiers::NONE));
    assert_eq!(editor.current_bufpos(), BufferPosition { line: 4, col: 4 });
    editor.handle_key_press(KeyEvent::new(KeyCode::Down, KeyModifiers::NONE));
    assert_eq!(editor.current_bufpos(), BufferPosition { line: 4, col: 4 });
    editor.handle_key_press(KeyEvent::new(KeyCode::Up, KeyModifiers::NONE));
    assert_eq!(editor.current_bufpos(), BufferPosition { line: 3, col: 0 });
    editor.handle_key_press(KeyEvent::new(KeyCode::Up, KeyModifiers::NONE));
    assert_eq!(editor.current_bufpos(), BufferPosition { line: 2, col: 4 });
    editor.handle_key_press(KeyEvent::new(KeyCode::Up, KeyModifiers::NONE));
    assert_eq!(editor.current_bufpos(), BufferPosition { line: 1, col: 3 });
    editor.handle_key_press(KeyEvent::new(KeyCode::Up, KeyModifiers::NONE));
    assert_eq!(editor.current_bufpos(), BufferPosition { line: 0, col: 4 });
    editor.handle_key_press(KeyEvent::new(KeyCode::Up, KeyModifiers::NONE));
    assert_eq!(editor.current_bufpos(), BufferPosition { line: 0, col: 4 });
    editor.handle_key_press(KeyEvent::new(KeyCode::Right, KeyModifiers::NONE));
    editor.handle_key_press(KeyEvent::new(KeyCode::Right, KeyModifiers::NONE));
    assert_eq!(editor.current_bufpos(), BufferPosition { line: 0, col: 6 });
    editor.handle_key_press(KeyEvent::new(KeyCode::Down, KeyModifiers::NONE));
    assert_eq!(editor.current_bufpos(), BufferPosition { line: 1, col: 3 });
    editor.handle_key_press(KeyEvent::new(KeyCode::Down, KeyModifiers::NONE));
    assert_eq!(editor.current_bufpos(), BufferPosition { line: 2, col: 6 });
    editor.handle_key_press(KeyEvent::new(KeyCode::Down, KeyModifiers::NONE));
    assert_eq!(editor.current_bufpos(), BufferPosition { line: 3, col: 0 });
    editor.handle_key_press(KeyEvent::new(KeyCode::Down, KeyModifiers::NONE));
    assert_eq!(editor.current_bufpos(), BufferPosition { line: 4, col: 5 });
    for _ in 0..3 {
        editor.handle_key_press(KeyEvent::new(KeyCode::Left, KeyModifiers::NONE));
    }
    assert_eq!(editor.current_bufpos(), BufferPosition { line: 4, col: 2 });
    editor.handle_key_press(KeyEvent::new(KeyCode::Up, KeyModifiers::NONE));
    assert_eq!(editor.current_bufpos(), BufferPosition { line: 3, col: 0 });
    editor.handle_key_press(KeyEvent::new(KeyCode::Up, KeyModifiers::NONE));
    assert_eq!(editor.current_bufpos(), BufferPosition { line: 2, col: 2 });
    editor.handle_key_press(KeyEvent::new(KeyCode::Up, KeyModifiers::NONE));
    assert_eq!(editor.current_bufpos(), BufferPosition { line: 1, col: 2 });
    editor.handle_key_press(KeyEvent::new(KeyCode::Up, KeyModifiers::NONE));
    assert_eq!(editor.current_bufpos(), BufferPosition { line: 0, col: 2 });
}

#[test]
fn test_home_and_end_insert_mode() {
    let mut editor = Editor::new(vec![Buffer::untitled()], Theme::default());
    let lines = ["abcdefg", "hij", "klmnop", "", "qrstu"];
    type_lines(&mut editor, &lines);
    assert_eq!(editor.current_bufpos(), BufferPosition { line: 4, col: 4 });
    editor.handle_key_press(KeyEvent::new(KeyCode::Char('i'), KeyModifiers::NONE));
    assert_eq!(editor.current_bufpos(), BufferPosition { line: 4, col: 4 });
    editor.handle_key_press(KeyEvent::new(KeyCode::Home, KeyModifiers::NONE));
    assert_eq!(editor.current_bufpos(), BufferPosition { line: 4, col: 0 });
    editor.handle_key_press(KeyEvent::new(KeyCode::Home, KeyModifiers::NONE));
    assert_eq!(editor.current_bufpos(), BufferPosition { line: 4, col: 0 });
    editor.handle_key_press(KeyEvent::new(KeyCode::End, KeyModifiers::NONE));
    assert_eq!(editor.current_bufpos(), BufferPosition { line: 4, col: 5 });
    editor.handle_key_press(KeyEvent::new(KeyCode::Up, KeyModifiers::NONE));
    assert_eq!(editor.current_bufpos(), BufferPosition { line: 3, col: 0 });
    editor.handle_key_press(KeyEvent::new(KeyCode::Up, KeyModifiers::NONE));
    assert_eq!(editor.current_bufpos(), BufferPosition { line: 2, col: 6 });
    editor.handle_key_press(KeyEvent::new(KeyCode::Up, KeyModifiers::NONE));
    assert_eq!(editor.current_bufpos(), BufferPosition { line: 1, col: 3 });
    editor.handle_key_press(KeyEvent::new(KeyCode::End, KeyModifiers::NONE));
    assert_eq!(editor.current_bufpos(), BufferPosition { line: 1, col: 3 });
    editor.handle_key_press(KeyEvent::new(KeyCode::Up, KeyModifiers::NONE));
    assert_eq!(editor.current_bufpos(), BufferPosition { line: 0, col: 7 });
    for _ in 0..4 {
        editor.handle_key_press(KeyEvent::new(KeyCode::Down, KeyModifiers::NONE));
    }
    assert_eq!(editor.current_bufpos(), BufferPosition { line: 4, col: 5 });
    // Unstick the cursor by moving to the left
    editor.handle_key_press(KeyEvent::new(KeyCode::Left, KeyModifiers::NONE));
    editor.handle_key_press(KeyEvent::new(KeyCode::Right, KeyModifiers::NONE));
    assert_eq!(editor.current_bufpos(), BufferPosition { line: 4, col: 5 });
    editor.handle_key_press(KeyEvent::new(KeyCode::Up, KeyModifiers::NONE));
    assert_eq!(editor.current_bufpos(), BufferPosition { line: 3, col: 0 });
    editor.handle_key_press(KeyEvent::new(KeyCode::Up, KeyModifiers::NONE));
    assert_eq!(editor.current_bufpos(), BufferPosition { line: 2, col: 5 });
    editor.handle_key_press(KeyEvent::new(KeyCode::Up, KeyModifiers::NONE));
    assert_eq!(editor.current_bufpos(), BufferPosition { line: 1, col: 3 });
    editor.handle_key_press(KeyEvent::new(KeyCode::Up, KeyModifiers::NONE));
    assert_eq!(editor.current_bufpos(), BufferPosition { line: 0, col: 5 });
    editor.handle_key_press(KeyEvent::new(KeyCode::Down, KeyModifiers::NONE));
    editor.handle_key_press(KeyEvent::new(KeyCode::Down, KeyModifiers::NONE));
    editor.handle_key_press(KeyEvent::new(KeyCode::Down, KeyModifiers::NONE));
    assert_eq!(editor.current_bufpos(), BufferPosition { line: 3, col: 0 });
    editor.handle_key_press(KeyEvent::new(KeyCode::End, KeyModifiers::NONE));
    assert_eq!(editor.current_bufpos(), BufferPosition { line: 3, col: 0 });
    // Unstick the cursor with Home
    editor.handle_key_press(KeyEvent::new(KeyCode::Home, KeyModifiers::NONE));
    assert_eq!(editor.current_bufpos(), BufferPosition { line: 3, col: 0 });
    editor.handle_key_press(KeyEvent::new(KeyCode::Down, KeyModifiers::NONE));
    assert_eq!(editor.current_bufpos(), BufferPosition { line: 4, col: 0 });
}
