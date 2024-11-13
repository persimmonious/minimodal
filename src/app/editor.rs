use std::{
    io::{self, stdout},
    rc::Rc,
};

use crossterm::{
    cursor::SetCursorStyle,
    event::{self, Event, KeyEvent, KeyEventKind},
    execute,
};
use ratatui::{
    layout::{Constraint, Direction, Layout, Position, Rect},
    style::{Style, Stylize},
    widgets::{Clear, Tabs},
    DefaultTerminal, Frame,
};

use crate::app::{
    buffer::{Buffer, BufferPosition},
    keymap::KeyMap,
    theme::Theme,
    ui::{leader_menu::LeaderMenu, status_bar::StatusBar, text_window::TextWindowState},
    ui::{leader_menu::SubMenu, Tab, TabState},
};

mod action_handlers;
pub mod actions;

#[derive(Debug, Clone)]
pub enum Mode {
    Normal,
    Command,
    Insert,
    Menu(SubMenu),
}

#[derive(Debug)]
struct EditorLayoutIndices {
    tabline: usize,
    tab: usize,
    menu: Option<usize>,
    status_bar: usize,
}
#[derive(Debug)]
pub struct Editor {
    active: bool,
    keymap: KeyMap,
    current_tab: usize,
    mode: Mode,
    tabs: Vec<Tab>,
    tab_states: Vec<TabState>,
    theme: Rc<Theme>,
}

const TABLINE_HEIGHT: u16 = 1;
const STATUS_LINE_HEIGHT: u16 = 1;

impl Editor {
    pub fn new(buffers: Vec<Buffer>, theme_struct: Theme) -> Self {
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

    pub fn draw(&mut self, frame: &mut Frame) {
        let (layout, indices) = match self.mode {
            Mode::Menu(ref submenu) => self.leader_menu_layout(&submenu, frame),
            _ => Self::standard_layout(frame),
        };

        let tabline = self.generate_tabline();
        frame.render_widget(tabline, layout[indices.tabline]);

        if let Mode::Menu(ref sub_menu) = self.mode {
            let mut tab_area = layout[indices.tab].clone();
            let menu_area = layout[indices
                .menu
                .expect("mismatch between editor mode and layout!")];

            tab_area.height += menu_area.height;
            frame.render_stateful_widget(
                self.tabs[self.current_tab].clone(),
                tab_area,
                &mut self.tab_states[self.current_tab],
            );

            frame.render_widget(Clear, menu_area);
            frame.render_widget(LeaderMenu::new(sub_menu, &self.theme), menu_area);
        } else {
            frame.render_stateful_widget(
                self.tabs[self.current_tab].clone(),
                layout[indices.tab],
                &mut self.tab_states[self.current_tab],
            );
        }

        let tab = &self.tab_states[self.current_tab];
        let status_bar = StatusBar::new(
            &tab.window_states,
            self.mode.clone(),
            Rc::downgrade(&self.theme),
        );
        frame.render_widget(&status_bar, layout[indices.status_bar]);
    }

    pub fn is_active(&self) -> bool {
        return self.active;
    }

    fn generate_tabline(&self) -> Tabs {
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
        Tabs::from_iter(buffer_titles)
            .select(self.current_tab)
            .divider("")
            .padding("", "")
            .style(tabs_style)
    }

    fn standard_layout(frame: &mut Frame) -> (Rc<[Rect]>, EditorLayoutIndices) {
        let layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints(vec![
                Constraint::Length(TABLINE_HEIGHT),
                Constraint::Min(1),
                Constraint::Length(STATUS_LINE_HEIGHT),
            ])
            .split(frame.area());
        let indices = EditorLayoutIndices {
            tabline: 0,
            tab: 1,
            menu: None,
            status_bar: 2,
        };
        return (layout, indices);
    }

    fn leader_menu_layout(
        &self,
        sub_menu: &SubMenu,
        frame: &mut Frame,
    ) -> (Rc<[Rect]>, EditorLayoutIndices) {
        let needed_height = LeaderMenu::required_height(sub_menu, frame.area().width);
        let layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints(vec![
                Constraint::Length(TABLINE_HEIGHT),
                Constraint::Min(1),
                Constraint::Length(needed_height),
                Constraint::Length(STATUS_LINE_HEIGHT),
            ])
            .split(frame.area());

        let indices = EditorLayoutIndices {
            tabline: 0,
            tab: 1,
            menu: Some(2),
            status_bar: 3,
        };
        return (layout, indices);
    }

    fn current_tabstate_mut(&mut self) -> &mut TabState {
        &mut self.tab_states[self.current_tab]
    }

    fn current_tabstate(&self) -> &TabState {
        &self.tab_states[self.current_tab]
    }

    fn current_winstate_mut(&mut self) -> &mut TextWindowState {
        &mut self.current_tabstate_mut().window_states
    }

    fn current_winstate(&self) -> &TextWindowState {
        &self.current_tabstate().window_states
    }

    fn current_bufpos(&self) -> BufferPosition {
        self.current_winstate().cursor.clone()
    }

    fn get_cursor_pos(&self) -> Position {
        let Position { x, y } = self.current_tabstate().get_cursor_pos();
        Position {
            y: y + TABLINE_HEIGHT,
            x,
        }
    }

    pub fn draw_cursor(&mut self, term: &mut DefaultTerminal) -> io::Result<()> {
        let pos = self.get_cursor_pos();
        term.set_cursor_position(pos)?;
        if let Mode::Insert = self.mode {
            execute!(stdout(), SetCursorStyle::SteadyBar)?;
        } else {
            execute!(stdout(), SetCursorStyle::SteadyBlock)?;
        }
        term.show_cursor()?;
        Ok(())
    }

    pub fn handle_input(&mut self) -> io::Result<()> {
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
}
