mod buffer;
mod theme;
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
use std::{ffi::OsString, fs, io, path::Path, rc::Rc};
use theme::Theme;
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
    theme: Rc<Theme>,
}

impl Editor {
    fn new(buffers: Vec<Buffer>, theme_struct: Theme) -> Self {
        let theme_rc = Rc::new(theme_struct);
        Editor {
            active: true,
            current_tab: 0,
            mode: Mode::Normal,
            theme: Rc::clone(&theme_rc),
            tabs: buffers.iter().map(|_| Tab::new()).collect(),
            tab_states: buffers
                .into_iter()
                .map(|buffer| TabState::new(buffer, Rc::downgrade(&theme_rc)))
                .collect(),
        }
    }

    fn draw(&mut self, frame: &mut Frame) {
        let layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints(vec![Constraint::Length(2), Constraint::Fill(1)])
            .split(frame.area());

        let buffer_titles = self.tab_states.iter().map(|tab| {
            tab.buffer
                .read_name()
                .map_or("Untitled", |x| x.try_into().expect("invalid file name!"))
        });
        let tabs_style = Style::default()
            .fg(self.theme.tabline_foreground)
            .bg(self.theme.tabline_background)
            .bold();
        let tabline = Tabs::from_iter(buffer_titles)
            .select(self.current_tab)
            .divider("")
            .style(tabs_style)
            .block(
                Block::new()
                    .borders(Borders::BOTTOM)
                    .border_type(BorderType::QuadrantInside)
                    .border_style(Style::default().fg(self.theme.tabline_border)),
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
            KeyCode::Char('$') => self.jump_to_EOL(),
            KeyCode::Char('0') => self.jump_to_home(),
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

    fn jump_to_EOL(&mut self) {
        self.tab_states[self.current_tab]
            .window_states
            .jump_to_EOL();
    }

    fn jump_to_home(&mut self) {
        self.tab_states[self.current_tab]
            .window_states
            .jump_to_home();
    }
}

pub fn initialize_buffers(config: &Config) -> Result<Vec<Buffer>, io::Error> {
    if config.file_names.is_empty() {
        return Ok(vec![Buffer::untitled()]);
    }
    let mut buffers: Vec<Buffer> = vec![];
    for name in &config.file_names {
        let path = Path::new(name);
        let name = path
            .file_name()
            .expect("cannot open a directory!")
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

    while editor.active {
        terminal.draw(|frame| editor.draw(frame))?;
        editor.handle_input()?;
    }
    return Ok(());
}
