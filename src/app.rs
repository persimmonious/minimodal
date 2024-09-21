use crate::config::Config;
use ratatui::{
    crossterm::event::{self, KeyCode, KeyEventKind},
    style::Stylize,
    widgets::Paragraph,
    DefaultTerminal,
};
use std::io;

pub fn run(terminal: &mut DefaultTerminal, config: Config) -> io::Result<()> {
    let sampletext = match config.file_name {
        None => format!("Test paragraph.\nPress 'q' to quit."),
        Some(file) => format!("File: {file}\nPress 'q' to quit."),
    };
    loop {
        terminal.draw(|frame| {
            let test_par = Paragraph::new(sampletext.to_owned()).white().on_blue();
            frame.render_widget(test_par, frame.area());
        })?;

        if let event::Event::Key(key) = event::read()? {
            if key.kind == KeyEventKind::Press && key.code == KeyCode::Char('q') {
                return Ok(());
            }
        }
    }
}
