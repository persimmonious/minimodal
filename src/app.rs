mod buffer;
use crate::config::Config;
use buffer::Buffer;
use ratatui::{
    crossterm::event::{self, KeyCode, KeyEventKind},
    style::Stylize,
    widgets::Paragraph,
    DefaultTerminal,
};
use std::{fs, io};

struct BufferPosition {
    line: usize,
    col: usize,
}

enum Mode {
    Normal,
    Command,
}

struct EditorState {
    current_buffer: usize,
    mode: Mode,
    cursor: BufferPosition,
}

impl EditorState {
    fn new() -> Self {
        EditorState {
            current_buffer: 0,
            mode: Mode::Normal,
            cursor: BufferPosition { line: 0, col: 0 },
        }
    }
}

pub fn initialize_buffers(config: &Config) -> Result<Vec<Buffer>, io::Error> {
    if config.file_names.is_empty() {
        return Ok(vec![Buffer::untitled()]);
    }
    let mut buffers: Vec<Buffer> = vec![];
    for name in &config.file_names {
        if fs::exists(name)? {
            buffers.push(Buffer::load(name, name)?);
        } else {
            buffers.push(Buffer::empty(name, name));
        }
    }
    Ok(buffers)
}

pub fn run(terminal: &mut DefaultTerminal, config: Config) -> io::Result<()> {
    let buffers = initialize_buffers(&config)?;
    let editor = EditorState::new();
    let sampletext = match buffers.len() {
        0 => format!("No files loaded.\nPress 'q' to quit."),
        _ => format!(
            "{}Press 'q' to quit.",
            buffers
                .iter()
                .map(|buf| buf.read_name().map_or("Untitled", |x| x))
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
