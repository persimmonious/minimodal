use super::*;

#[test]
fn test_create_empty_editor() {
    let editor = Editor::new(vec![Buffer::untitled()], Theme::default());
    assert_eq!(editor.current_bufpos(), BufferPosition { line: 0, col: 0 });
    assert!(editor
        .current_tabstate()
        .buffer
        .borrow()
        .read_name()
        .is_none());
    assert!(editor.current_tabstate().buffer.borrow().path().is_none());
}
