use std::{
    cell::{Ref, RefMut},
    io::{self, stdout},
    rc::Rc,
};

use crossterm::{
    cursor::SetCursorStyle,
    event::{self, Event, KeyEvent, KeyEventKind},
    execute,
};
use ratatui::{
    layout::{Constraint, Direction, Layout, Margin, Position, Rect},
    style::{Style, Stylize},
    widgets::{Clear, Tabs},
    DefaultTerminal, Frame,
};

use crate::app::{
    buffer::{Buffer, BufferPosition},
    cleanup::{graceful_exit, CleanUnwrap},
    keymap::KeyMap,
    theme::Theme,
    ui::{
        floating_window::FloatingContent,
        leader_menu::{LeaderMenu, SubMenu},
        status_bar::StatusBar,
        text_window::{selection::Selection, TextWindowState},
        Tab, TabState,
    },
};

mod action_handlers;
pub mod actions;

#[derive(Debug, Clone, PartialEq)]
pub enum Mode {
    Normal,
    Command,
    Insert,
    Select(Selection),
}

#[derive(Debug)]
struct EditorLayoutIndices {
    tabline: usize,
    tab: usize,
    menu: Option<usize>,
    status_bar: usize,
}

pub struct Editor {
    active: bool,
    keymap: KeyMap,
    current_tab: usize,
    mode: Mode,
    tabs: Vec<Tab>,
    tab_states: Vec<TabState>,
    theme: Rc<Theme>,
    lower_menu: Option<SubMenu>,
    floating_window: Option<Box<dyn FloatingContent>>,
}

const TABLINE_HEIGHT: u16 = 1;
const STATUS_LINE_HEIGHT: u16 = 1;
const FLOATING_WINDOW_SPACE_FRACTION: f64 = 0.8;

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
                .map(|buffer| TabState::new(buffer, Rc::downgrade(&theme_rc), Mode::Normal))
                .collect(),
            lower_menu: None,
            floating_window: None,
        }
    }

    pub fn draw(&mut self, frame: &mut Frame) {
        // Must be the first step to ensure that other widgets are in the right
        // mode
        self.propagate_mode();

        let (layout, indices) = match self.lower_menu {
            Some(ref submenu) => self.leader_menu_layout(submenu, frame),
            _ => Self::standard_layout(frame),
        };

        let tabline = self.generate_tabline();
        frame.render_widget(tabline, layout[indices.tabline]);

        if let Some(ref sub_menu) = self.lower_menu {
            let mut tab_area = layout[indices.tab];
            let menu_area = layout[indices
                .menu
                .clean_expect("mismatch between editor mode and layout!")];

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
                self.current_tabstate_mut(),
            );
        }

        let tab = &self.tab_states[self.current_tab];
        let status_bar = StatusBar::new(
            &tab.window_states,
            self.get_mode().clone(),
            Rc::downgrade(&self.theme),
        );
        frame.render_widget(&status_bar, layout[indices.status_bar]);

        if let Some(ref floating) = self.floating_window {
            let area = Self::floating_window_area(frame);
            floating.render(&area, frame, self.theme.clone());
        }
    }

    pub fn propagate_mode(&mut self) {
        let mode = self.get_mode().to_owned();
        for tabstate in &mut self.tab_states {
            tabstate.propagate_mode(mode.clone());
        }
    }

    pub fn is_active(&self) -> bool {
        self.active
    }

    fn generate_tabline(&self) -> Tabs {
        let buffer_titles = self
            .tab_states
            .iter()
            .map(|tab| -> String {
                tab.buffer
                    .borrow()
                    .read_name()
                    .map_or("Untitled", |x| {
                        let tab_str = x.try_into();
                        if tab_str.is_err() {
                            let err = tab_str.unwrap_err();
                            graceful_exit(Some(&format!("file name is not valid Unicode! {err}")));
                        }
                        tab_str.unwrap()
                    })
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
        (layout, indices)
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
        (layout, indices)
    }

    fn floating_window_area(frame: &Frame) -> Rect {
        let full_area = frame.area();
        let height = full_area.height;
        let vert_margin =
            (height as f64 * (1.0 - FLOATING_WINDOW_SPACE_FRACTION) / 2.0).floor() as u16;
        let width = full_area.width;
        let hor_margin =
            (width as f64 * (1.0 - FLOATING_WINDOW_SPACE_FRACTION) / 2.0).floor() as u16;
        let margin = Margin::new(hor_margin, vert_margin);
        Rect::inner(full_area, margin)
    }

    pub(crate) fn get_mode(&self) -> &Mode {
        &self.mode
    }

    pub(crate) fn current_tabstate_mut(&mut self) -> &mut TabState {
        &mut self.tab_states[self.current_tab]
    }

    pub(crate) fn current_tabstate(&self) -> &TabState {
        &self.tab_states[self.current_tab]
    }

    fn current_winstate_mut(&mut self) -> &mut TextWindowState {
        &mut self.current_tabstate_mut().window_states
    }

    fn current_winstate(&self) -> &TextWindowState {
        &self.current_tabstate().window_states
    }

    pub(crate) fn current_buffer(&self) -> Ref<Buffer> {
        self.current_tabstate().buffer.borrow()
    }

    pub(crate) fn current_buffer_mut(&self) -> RefMut<Buffer> {
        self.current_tabstate().buffer.borrow_mut()
    }

    pub(crate) fn current_bufpos(&self) -> BufferPosition {
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
        if let Mode::Insert = self.get_mode() {
            execute!(stdout(), SetCursorStyle::SteadyBar)?;
        } else {
            execute!(stdout(), SetCursorStyle::SteadyBlock)?;
        }
        term.show_cursor()?;
        Ok(())
    }

    pub(crate) fn handle_input(&mut self) -> io::Result<()> {
        match event::read()? {
            Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                self.handle_key_press(key_event)
            }
            _ => {}
        };
        Ok(())
    }

    pub(crate) fn handle_key_press(&mut self, key: KeyEvent) {
        if let Some(window) = &mut self.floating_window {
            if let Some(callback) = window.handle_input(&key) {
                callback(self);
            }
            return;
        }
        let bound_action = if let Some(ref menu) = self.lower_menu {
            self.keymap.handle_menu_input(&key, menu)
        } else {
            self.keymap.handle_key(&key, self.get_mode())
        };
        if let Some(action) = bound_action {
            self.execute_editor_action(action);
        }
    }

    pub(crate) fn clear_floating_window(&mut self) {
        self.floating_window = None;
    }

    pub(crate) fn update_selection(&mut self) {
        let bufpos = self.current_bufpos();
        if let Mode::Select(sel) = &mut self.mode {
            sel.moving_point = bufpos;
        }
    }

    pub(crate) fn switch_mode(&mut self, mode: Mode) {
        self.mode = mode;
    }
}
