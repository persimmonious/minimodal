use super::{
    buffer::{HorizontalDirection, RectilinearDirection, VerticalDirection},
    Mode,
};

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
    InsertLineBreak,
    InsertChar(char),
    MoveToHomeAndEnterInsert,
    MoveCursor(Mode, RectilinearDirection),
    InsertNewLine(VerticalDirection),
    ReplaceLine,
    RemoveChar,
    SaveBuffer,
}
