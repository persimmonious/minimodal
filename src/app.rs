use crate::config::Config;
use ratatui::{
    crossterm::event::{self, KeyCode, KeyEventKind},
    style::Stylize,
    widgets::Paragraph,
    DefaultTerminal,
};
use std::io;

pub fn run(terminal: &mut DefaultTerminal, config: Config) -> io::Result<()> {
    let sampletext = match config.file_names.len() {
        0 => format!("No files loaded.\nPress 'q' to quit."),
        _ => format!(
            "{}Press 'q' to quit.",
            config
                .file_names
                .iter()
                .map(|name| format!("File: {name}\n"))
                .fold(String::new(), |mut acc, x| {
                    acc.push_str(&x);
                    acc
                })
        ),
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
