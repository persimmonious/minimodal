use clap::{arg, command};
use ratatui::{
    crossterm::event::{self, KeyCode, KeyEventKind},
    style::Stylize,
    widgets::Paragraph,
    DefaultTerminal,
};
use std::error::Error;
use std::io;

#[derive(Debug)]
pub struct Config {
    pub file_name: Option<String>,
}

impl Config {
    fn new() -> Config {
        return Config { file_name: None };
    }
}

pub fn parse_command_line() -> Result<Config, Box<dyn Error>> {
    let mut config = Config::new();
    let arg_matches = command!()
        .arg(arg!(--file <FILENAME>).required(false))
        .get_matches();

    let file_name_arg = arg_matches.get_one::<String>("file");
    match file_name_arg {
        Some(file_name_str) => {
            config.file_name = Some(file_name_str.to_owned());
        }
        None => (),
    }
    return Ok(config);
}

pub fn run(terminal: &mut DefaultTerminal, config: Config) -> io::Result<()> {
    let sampletext = match config.file_name {
        None => format!("Test paragraph.\nPress 'q' to quit."),
        Some(file) => format!("File: {file}\nPress 'q' to quit.")
    };
    loop {
        terminal.draw(|frame| {
            let test_par = Paragraph::new(sampletext.to_owned())
                .white()
                .on_blue();
            frame.render_widget(test_par, frame.area());
        })?;

        if let event::Event::Key(key) = event::read()? {
            if key.kind == KeyEventKind::Press && key.code == KeyCode::Char('q') {
                return Ok(());
            }
        }
    }
}
