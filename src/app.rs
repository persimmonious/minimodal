mod buffer;
use crate::config::Config;
use buffer::{Buffer, HorizontalDirection as Horizontal};
use ratatui::{
    buffer::Buffer as RatBuffer,
    crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind},
    layout::{Alignment, Rect},
    text::{Line, Span, Text},
    widgets::{
        block::{Position, Title},
        Block, Paragraph, Widget,
    },
    DefaultTerminal, Frame,
};
use std::{fs, io};

#[derive(Debug)]
struct BufferPosition {
    line: usize,
    col: usize,
}

#[derive(Debug)]
enum Mode {
    Normal,
    Command,
}

#[derive(Debug)]
struct Editor {
    pub active: bool,
    current_buffer: usize,
    mode: Mode,
    cursor: BufferPosition,
    buffers: Vec<Buffer>,
}

impl Editor {
    fn new(buffers: Vec<Buffer>) -> Self {
        Editor {
            active: true,
            current_buffer: 0,
            mode: Mode::Normal,
            cursor: BufferPosition { line: 0, col: 0 },
            buffers,
        }
    }

    fn draw(&self, frame: &mut Frame) {
        frame.render_widget(self, frame.area());
    }

    fn handle_input(&mut self) -> io::Result<()> {
        match event::read()? {
            Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                self.handle_key_press(key_event)
            }
            _ => {}
        };
        Ok(())
    }

    fn handle_key_press(&mut self, key: KeyEvent) {
        match key.code {
            KeyCode::Char('q') => self.exit(),
            KeyCode::Tab => self.cycle_buffer(Horizontal::Forwards),
            KeyCode::BackTab => self.cycle_buffer(Horizontal::Backwards),
            _ => {}
        }
    }

    fn exit(&mut self) {
        self.active = false;
    }

    fn cycle_buffer(&mut self, dir: Horizontal) {
        self.current_buffer = match dir {
            Horizontal::Forwards => (self.current_buffer + 1) % self.buffers.len(),
            Horizontal::Backwards => match self.current_buffer {
                0 => self.buffers.len() - 1,
                current => (current - 1) % self.buffers.len(),
            },
        }
    }
}

impl Widget for &Editor {
    fn render(self, area: Rect, buf: &mut RatBuffer) {
        let buffer_titles: Vec<Span> = self
            .buffers
            .iter()
            .map(|buf| buf.read_name().map_or("Untitled", |x| x))
            .map(|buf_name| format!(" {buf_name} |").into())
            .collect();
        let titlebar = Title::from(Line::from(buffer_titles));
        let block = Block::new().title(titlebar.alignment(Alignment::Left).position(Position::Top));
        let buffer = &self.buffers[self.current_buffer];
        let lines: Vec<_> = buffer
            .lines
            .iter()
            .map(|line| line.into())
            .map(|line: Span| Line::from(line))
            .collect();
        let text = Text::from(lines);

        Paragraph::new(text)
            .left_aligned()
            .block(block)
            .render(area, buf);
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
    let mut editor = Editor::new(buffers);

    while editor.active {
        terminal.draw(|frame| editor.draw(frame))?;
        editor.handle_input()?;
    }
    return Ok(());
}
