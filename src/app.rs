mod buffer;
mod ui;
use crate::config::Config;
use buffer::{Buffer, HorizontalDirection as Horizontal, RectilinearDirection as Rectilinear};
use ratatui::{
    crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind},
    layout::{Constraint, Direction, Layout},
    style::Stylize,
    style::{Color, Style},
    widgets::{block::BorderType, Block, Borders, Tabs},
    DefaultTerminal, Frame,
};
use std::{fs, io};
use ui::{Tab, TabState};

#[derive(Debug)]
enum Mode {
    Normal,
    Command,
}

#[derive(Debug)]
struct Editor {
    pub active: bool,
    current_tab: usize,
    mode: Mode,
    tabs: Vec<Tab>,
    tab_states: Vec<TabState>,
}

impl Editor {
    fn new(buffers: Vec<Buffer>) -> Self {
        Editor {
            active: true,
            current_tab: 0,
            mode: Mode::Normal,
            tabs: buffers.iter().map(|_| Tab::new()).collect(),
            tab_states: buffers
                .into_iter()
                .map(|buffer| TabState::new(buffer))
                .collect(),
        }
    }

    fn draw(&mut self, frame: &mut Frame) {
        let layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints(vec![Constraint::Length(2), Constraint::Fill(1)])
            .split(frame.area());

        let buffer_titles = self
            .tab_states
            .iter()
            .map(|tab| tab.buffer.read_name().map_or("Untitled", |x| x));
        let tabs_style = Style::default()
            .fg(Color::Rgb(255, 190, 140))
            .bg(Color::Black)
            .bold();
        let tabline = Tabs::from_iter(buffer_titles)
            .select(self.current_tab)
            .style(tabs_style)
            .block(
                Block::new()
                    .borders(Borders::BOTTOM)
                    .border_type(BorderType::QuadrantInside)
                    .border_style(Style::default().fg(Color::Rgb(180, 120, 80))),
            );

        frame.render_widget(tabline, layout[0]);
        frame.render_stateful_widget(
            self.tabs[self.current_tab].clone(),
            layout[1],
            &mut self.tab_states[self.current_tab],
        );
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
            KeyCode::Tab => self.cycle_tab(Horizontal::Forwards),
            KeyCode::BackTab => self.cycle_tab(Horizontal::Backwards),
            KeyCode::Char('j') => self.move_cursor(Rectilinear::Down),
            KeyCode::Char('k') => self.move_cursor(Rectilinear::Up),
            KeyCode::Char('h') => self.move_cursor(Rectilinear::Left),
            KeyCode::Char('l') => self.move_cursor(Rectilinear::Right),
            _ => {}
        }
    }

    fn exit(&mut self) {
        self.active = false;
    }

    fn cycle_tab(&mut self, dir: Horizontal) {
        self.current_tab = match dir {
            Horizontal::Forwards => (self.current_tab + 1) % self.tabs.len(),
            Horizontal::Backwards => match self.current_tab {
                0 => self.tabs.len() - 1,
                current => (current - 1) % self.tabs.len(),
            },
        }
    }

    fn move_cursor(&mut self, dir: Rectilinear) {
        self.tab_states[self.current_tab]
            .window_states
            .move_cursor(dir);
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
