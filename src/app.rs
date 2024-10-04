mod buffer;
mod theme;
mod ui;
use crate::config::Config;
use buffer::{Buffer, HorizontalDirection as Horizontal, RectilinearDirection as Rectilinear};
use ratatui::{
    crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind},
    layout::{Constraint, Direction, Layout},
    style::{Style, Stylize},
    widgets::Tabs,
    DefaultTerminal, Frame,
};
use std::{io, path::Path, rc::Rc};
use theme::Theme;
use ui::text_window::TextWindowState;
use ui::{status_bar::StatusBar, Tab, TabState};

#[derive(Debug, Clone)]
pub enum Mode {
    Normal,
    Command,
    Insert,
    Menu,
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
            .constraints(vec![
                Constraint::Length(1),
                Constraint::Fill(1),
                Constraint::Length(1),
            ])
            .split(frame.area());

        let buffer_titles = self
            .tab_states
            .iter()
            .map(|tab| -> String {
                tab.buffer
                    .borrow()
                    .read_name()
                    .map_or("Untitled", |x| x.try_into().expect("invalid file name!"))
                    .to_owned()
            })
            .map(|name| format!(" {name} "));
        let tabs_style = Style::default()
            .fg(self.theme.tabline_foreground)
            .bg(self.theme.tabline_background)
            .bold();
        let tabline = Tabs::from_iter(buffer_titles)
            .select(self.current_tab)
            .divider("")
            .padding("", "")
            .style(tabs_style);

        frame.render_widget(tabline, layout[0]);
        frame.render_stateful_widget(
            self.tabs[self.current_tab].clone(),
            layout[1],
            &mut self.tab_states[self.current_tab],
        );

        let tab = &self.tab_states[self.current_tab];
        let status_bar = StatusBar::new(
            &tab.window_states,
            self.mode.clone(),
            Rc::downgrade(&self.theme),
        );
        frame.render_widget(&status_bar, layout[2]);
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
        match self.mode {
            Mode::Normal => self.handle_key_press_normal(key),
            Mode::Command => todo!(),
            Mode::Insert => self.handle_key_press_insert(key),
            Mode::Menu => self.handle_key_press_menu(key),
        }
    }

    fn handle_key_press_normal(&mut self, key: KeyEvent) {
        match key.code {
            KeyCode::Char(' ') => self.enter_menu(),
            KeyCode::Char('i') => self.enter_insert(),
            KeyCode::Char('I') => {
                self.jump_to_home();
                self.enter_insert();
            }
            KeyCode::Char('S') => self.replace_line(),
            KeyCode::Tab => self.cycle_tab(Horizontal::Forwards),
            KeyCode::BackTab => self.cycle_tab(Horizontal::Backwards),
            KeyCode::Char('j') => self.move_cursor(Rectilinear::Down),
            KeyCode::Char('k') => self.move_cursor(Rectilinear::Up),
            KeyCode::Char('h') => self.move_cursor(Rectilinear::Left),
            KeyCode::Char('l') => self.move_cursor(Rectilinear::Right),
            KeyCode::Char('$') => self.jump_to_EOL(),
            KeyCode::Char('0') => self.jump_to_home(),
            KeyCode::Char('G') => self.jump_to_last_line(),
            _ => {}
        }
    }

    fn handle_key_press_insert(&mut self, key: KeyEvent) {
        match key.code {
            KeyCode::Esc => self.exit_insert(),
            KeyCode::Char(c) => self.insert_char(c),
            _ => {}
        }
    }

    fn handle_key_press_menu(&mut self, key: KeyEvent) {
        match key.code {
            KeyCode::Esc | KeyCode::Char(' ') => self.exit_menu(),
            KeyCode::Char('q') => self.exit(),
            _ => {}
        }
    }

    fn current_tabstate(&mut self) -> &mut TabState {
        &mut self.tab_states[self.current_tab]
    }

    fn current_winstate(&mut self) -> &mut TextWindowState {
        &mut self.current_tabstate().window_states
    }

    fn enter_menu(&mut self) {
        self.mode = Mode::Menu;
    }

    fn exit_menu(&mut self) {
        self.mode = Mode::Normal;
    }

    fn enter_insert(&mut self) {
        self.mode = Mode::Insert;
    }

    fn exit_insert(&mut self) {
        self.mode = Mode::Normal;
        self.current_winstate().snap_to_EOL();
    }

    fn insert_char(&mut self, c: char) {
        self.current_tabstate().insert_char(c);
    }

    fn replace_line(&mut self) {
        self.enter_insert();
        self.current_tabstate().replace_line();
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

    fn jump_to_last_line(&mut self) {
        self.tab_states[self.current_tab]
            .window_states
            .jump_to_last_line();
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
