use super::{
    buffer::{HorizontalDirection, RectilinearDirection, VerticalDirection},
    Mode,
};

#[derive(Debug, Clone)]
pub enum EditorAction {
    Append,
    AppendAtEOL,
    Back,
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
    InsertLineBreak,
    InsertNewLine(VerticalDirection),
    MoveToHomeAndEnterInsert,
    MoveCursor(Mode, RectilinearDirection),
    NextLine,
    ReplaceLine,
    RemoveChar,
    SaveBuffer,
}
