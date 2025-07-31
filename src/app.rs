pub(crate) mod buffer;
pub(crate) mod cleanup;
pub(crate) mod editor;
pub(crate) mod keymap;
pub(crate) mod theme;
pub(crate) mod ui;

use crate::config::Config;
use buffer::Buffer;
use cleanup::{graceful_exit, CleanUnwrap};
use crossterm::{
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use editor::{actions::EditorAction, Editor};
use ratatui::DefaultTerminal;
use std::{
    io::{self, stdout},
    path::Path,
};
use theme::Theme;

pub fn initialize_buffers(config: &Config) -> Result<Vec<Buffer>, io::Error> {
    if config.file_names.is_empty() {
        return Ok(vec![Buffer::untitled()]);
    }
    let mut buffers: Vec<Buffer> = vec![];
    for name in &config.file_names {
        let path = Path::new(name);
        if path.is_dir() {
            graceful_exit(Some("Opening directories is not supported"))
        }
        let name = path
            .file_name()
            .clean_expect("File name is not valid")
            .to_owned();
        if path.try_exists()? {
            buffers.push(Buffer::load(name, path.into())?);
        } else {
            buffers.push(Buffer::empty(name, path.into()));
        }
    }
    Ok(buffers)
}

pub fn run(terminal: &mut DefaultTerminal, config: Config) -> io::Result<()> {
    let buffers = initialize_buffers(&config)?;
    let mut editor = Editor::new(buffers, Theme::default());

    enable_raw_mode()?;
    execute!(stdout(), EnterAlternateScreen)?;
    while editor.is_active() {
        terminal.draw(|frame| editor.draw(frame))?;
        editor.draw_cursor(terminal)?;
        editor.handle_input()?;
    }
    execute!(stdout(), LeaveAlternateScreen)?;
    disable_raw_mode()?;
    Ok(())
}
