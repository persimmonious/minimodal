use crate::app::buffer::{HorizontalDirection, RectilinearDirection, VerticalDirection};
use crate::app::editor::Mode;
use crate::app::ui::floating_window::FloatingContent;

#[allow(clippy::upper_case_acronyms)]
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
    EnterSelect,
    EOL,
    ExitEditor,
    ExitInsert,
    ExitMenu,
    ExitSelect,
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
