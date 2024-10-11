use super::{
    buffer::{HorizontalDirection, RectilinearDirection},
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
    InsertChar(char),
    MoveToHomeAndEnterInsert,
    MoveCursor(Mode, RectilinearDirection),
    ReplaceLine,
    SaveBuffer,
}
