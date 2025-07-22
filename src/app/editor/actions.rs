use crate::app::buffer::{HorizontalDirection, RectilinearDirection, VerticalDirection};
use crate::app::editor::Mode;
use crate::app::ui::floating_window::FloatingContent;

#[derive(Clone)]
pub enum EditorAction {
    Append,
    AppendAtEOL,
    Back,
    CycleTab(HorizontalDirection),
    EndOfBuffer,
    EnterInsert,
    EnterFloatingMenu(Box<dyn FloatingContent>),
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
    RemoveChar(HorizontalDirection),
    SaveBuffer,
}
