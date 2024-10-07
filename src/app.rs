mod buffer;
mod keymap;
mod theme;
mod ui;
use crate::config::Config;
use buffer::{
    Buffer, BufferPosition, HorizontalDirection as Horizontal, RectilinearDirection as Rectilinear,
};
use crossterm::{
    cursor::{MoveToColumn, MoveToRow, SetCursorStyle},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use keymap::{EditorAction, KeyMap};
use ratatui::{
    crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind},
    layout::{Constraint, Direction, Layout, Position},
    style::{Style, Stylize},
    widgets::Tabs,
    DefaultTerminal, Frame,
};
use std::{
    io::{self, stdout},
    path::Path,
    rc::Rc,
};
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
    active: bool,
    keymap: KeyMap,
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
            keymap: KeyMap::default(),
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

    fn draw_cursor(&mut self, term: &mut DefaultTerminal) {
        let (row, col) = self.current_tabstate().get_cursor_pos();
        let row: u16 = row.try_into().expect("row number is too large");
        let pos = Position {
            y: row + 1,
            x: col as u16,
        };

        term.set_cursor_position(pos).unwrap();
        if let Mode::Insert = self.mode {
            execute!(stdout(), SetCursorStyle::SteadyBar).unwrap();
        } else {
            execute!(stdout(), SetCursorStyle::SteadyBlock).unwrap();
        }
        term.show_cursor().unwrap();
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
        if let Some(action) = self.keymap.handle_key(&key, &self.mode) {
            self.execute_editor_action(action);
        }
    }

    fn execute_editor_action(&mut self, action: EditorAction) {
        match action {
            EditorAction::EnterInsert => self.enter_insert(),
            EditorAction::EnterMenu => self.enter_menu(),
            EditorAction::ExitInsert => self.exit_insert(),
            EditorAction::ExitEditor => self.exit(),
            EditorAction::ExitMenu => self.exit_menu(),
            EditorAction::InsertChar(c) => self.insert_char(c),
            EditorAction::MoveToHomeAndEnterInsert => {
                self.jump_to_home();
                self.enter_insert();
            }
            EditorAction::ReplaceLine => self.replace_line(),
            EditorAction::CycleTab(dir) => self.cycle_tab(dir),
            EditorAction::MoveCursor(mode, dir) => match mode {
                Mode::Normal => self.move_cursor(dir),
                _ => (),
            },
            EditorAction::EOL => self.sticky_jump_to_EOL(),
            EditorAction::Home => self.jump_to_home(),
            EditorAction::EndOfBuffer => self.jump_to_last_line(),
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

    fn sticky_jump_to_EOL(&mut self) {
        self.tab_states[self.current_tab]
            .window_states
            .sticky_jump_to_EOL();
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

    enable_raw_mode()?;
    execute!(stdout(), EnterAlternateScreen)?;
    while editor.active {
        terminal.draw(|frame| editor.draw(frame))?;
        editor.draw_cursor(terminal);
        editor.handle_input()?;
    }
    execute!(stdout(), LeaveAlternateScreen)?;
    disable_raw_mode()?;
    return Ok(());
}
