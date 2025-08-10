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
    EnterVisual,
    EOL,
    ExitEditor,
    ExitInsert,
    ExitMenu,
    ExitVisual,
    Home,
    InsertChar(char),
    InsertLineBreak,
    InsertNewLine(VerticalDirection),
    MoveCursor(Mode, RectilinearDirection),
    MoveToHomeAndEnterInsert,
    NextLine,
    RemoveChar(HorizontalDirection),
    ReplaceLine,
    SaveBuffer,
    SwitchToMode(Mode),
}
