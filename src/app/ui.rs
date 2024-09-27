mod line_numbers;
use super::{
    buffer::{Buffer, BufferPosition, RectilinearDirection as Rectilinear},
    theme::Theme,
};
use line_numbers::LineNumberType::{Absolute, Relative};
use line_numbers::LineNumbers;
use ratatui::{
    buffer::Buffer as TUI_Buffer,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Modifier, Style, Styled, Stylize},
    text::{Line, Span},
    widgets::{Block, Clear, Paragraph, StatefulWidget, Widget},
};
use std::{
    cmp::min,
    rc::{Rc, Weak},
};

#[derive(Debug, Clone)]
pub struct Tab {}

#[derive(Debug)]
pub struct TabState {
    pub window_states: TextWindowState,
    pub buffer: Rc<Buffer>,
    pub windows: TextWindow,
    current_window: usize,
}

impl TabState {
    pub fn new(buf: Buffer, theme: Weak<Theme>) -> Self {
        let buf_rc = Rc::new(buf);
        return TabState {
            buffer: Rc::clone(&buf_rc),
            window_states: TextWindowState::new(Rc::downgrade(&buf_rc), theme.clone()),
            windows: TextWindow::new(Rc::downgrade(&buf_rc), theme.clone()),
            current_window: 0,
        };
    }
}

impl Tab {
    pub fn new() -> Self {
        Tab {}
    }
}

impl StatefulWidget for Tab {
    type State = TabState;

    fn render(self, area: Rect, buf: &mut TUI_Buffer, state: &mut Self::State) {
        state
            .windows
            .clone()
            .render(area, buf, &mut state.window_states);
    }
}

#[derive(Debug, Clone)]
pub struct TextWindow {
    buffer: Weak<Buffer>,
    theme: Weak<Theme>,
}

#[derive(Debug)]
pub struct TextWindowState {
    pub top_line: usize,
    pub leftmost_col: usize,
    pub last_height: usize,
    pub last_width: usize,
    pub cur_vertical_percent: f32,
    pub cursor: BufferPosition,
    buffer: Weak<Buffer>,
    theme: Weak<Theme>,
}

impl TextWindowState {
    pub fn new(buffer: Weak<Buffer>, theme: Weak<Theme>) -> Self {
        return TextWindowState {
            top_line: 0,
            leftmost_col: 0,
            last_height: 2,
            last_width: 2,
            cur_vertical_percent: 0.0,
            cursor: BufferPosition { line: 0, col: 0 },
            buffer,
            theme,
        };
    }

    pub fn move_cursor(&mut self, dir: Rectilinear) {
        match dir {
            Rectilinear::Up => {
                if self.cursor.line <= 0 {
                    return;
                }
                let mut relative_line = self.cursor.line - self.top_line;
                let line_length = self.line_length(self.cursor.line);
                let cur_at_EOL = line_length == 0 || self.cursor.col == line_length - 1;
                self.cursor.line -= 1;
                if self.cursor.line < self.top_line {
                    self.cur_vertical_percent = 0.0;
                    self.top_line = self.cursor.line;
                } else {
                    relative_line -= 1;
                    self.cur_vertical_percent =
                        relative_line as f32 / (self.last_height - 1) as f32;
                }
                let new_line_length = self.line_length(self.cursor.line);
                if self.cursor.col >= new_line_length || cur_at_EOL {
                    self.jump_to_EOL();
                }
            }

            Rectilinear::Down => {
                if self.cursor.line + 1 >= self.lines_count() {
                    return;
                }
                let mut relative_line = self.cursor.line - self.top_line;
                let line_length = self.line_length(self.cursor.line);
                let cur_at_EOL = line_length == 0 || self.cursor.col == line_length - 1;
                self.cursor.line += 1;
                // float comparison OK here because it is exact
                if self.cur_vertical_percent == 1.0 {
                    self.top_line += 1;
                } else {
                    relative_line += 1;
                    self.cur_vertical_percent =
                        relative_line as f32 / (self.last_height - 1) as f32;
                }
                let new_line_length = self.line_length(self.cursor.line);
                if self.cursor.col >= new_line_length || cur_at_EOL {
                    self.jump_to_EOL();
                }
            }
            Rectilinear::Right => {
                let line_length = self.line_length(self.cursor.line);
                if self.cursor.col + 1 >= line_length {
                    return;
                }
                self.cursor.col += 1;
                if self.cursor.col >= self.leftmost_col + self.last_width {
                    self.leftmost_col += 1;
                }
            }
            Rectilinear::Left => {
                if self.cursor.col <= 0 {
                    return;
                }
                self.cursor.col -= 1;
                if self.cursor.col < self.leftmost_col {
                    self.leftmost_col = self.cursor.col;
                }
            }
        }
    }

    pub fn jump_to_EOL(&mut self) {
        let line_length = self.line_length(self.cursor.line);
        if line_length == 0 {
            self.cursor.col = 0;
            self.leftmost_col = 0;
            return;
        }
        self.cursor.col = line_length - 1;
        let to_the_right = self.cursor.col >= self.leftmost_col + self.last_width;
        let out_of_bounds = to_the_right || self.cursor.col < self.leftmost_col;
        if !out_of_bounds {
            return;
        }
        if to_the_right {
            self.leftmost_col = self.cursor.col + 1 - self.last_width;
        } else if self.cursor.col >= self.last_width {
            self.leftmost_col = self.cursor.col + 1 - self.last_width;
        } else {
            self.leftmost_col = 0;
        }
    }

    pub fn jump_to_home(&mut self) {
        self.cursor.col = 0;
        self.leftmost_col = 0;
    }

    fn lines_count(&self) -> usize {
        self.buffer
            .upgrade()
            .expect("counting lines in a dead buffer!")
            .lines
            .len()
    }

    fn line_length(&self, line: usize) -> usize {
        self.buffer
            .upgrade()
            .expect("checking line length in a dead buffer!")
            .lines[line]
            .len()
    }
}

impl TextWindow {
    pub fn new(buffer: Weak<Buffer>, theme: Weak<Theme>) -> TextWindow {
        TextWindow { buffer, theme }
    }

    fn build_lines(&self, height: u16, width: usize, state: &mut TextWindowState) -> Vec<Line> {
        let buffer = self
            .buffer
            .upgrade()
            .expect("building lines from a dead buffer!");
        let theme = self.theme.upgrade().expect("referencing dropped theme!");

        state.last_height = height.into();
        state.last_width = width;
        let cursor_rel_line: usize =
            (state.cur_vertical_percent * (height - 1) as f32).floor() as usize;
        let top_line: usize = state.cursor.line - cursor_rel_line;
        let last_line: usize = min(top_line + height as usize, state.lines_count());
        let line_style = Style::default()
            .fg(theme.text_foreground)
            .bg(theme.text_background);
        return buffer.lines[top_line..last_line]
            .iter()
            .map(|line| {
                if state.leftmost_col < line.len() {
                    line[state.leftmost_col..].to_string()
                } else {
                    "".to_string()
                }
            })
            .map(|line| Line::styled(format!("{line: <width$}"), line_style))
            .collect();
    }

    fn highlight_cursor(&self, lines: &mut Vec<Line>, state: &mut TextWindowState) {
        if lines.is_empty() {
            lines.push(Line::from(" "));
        }

        if state.cursor.line < state.top_line {
            return;
        }
        let line = state.cursor.line - state.top_line;
        if line >= lines.len() {
            return;
        }

        let theme = self.theme.upgrade().expect("referencing dropped theme!");
        let col = state.cursor.col - state.leftmost_col;
        let line_style = Style::default()
            .bg(theme.selected_line_background)
            .fg(theme.selected_line_foreground);
        let cur_style = line_style.add_modifier(Modifier::REVERSED);

        let old_line: String = lines[line].to_owned().into();
        if old_line.is_empty() {
            lines[line] = Line::styled(" ", cur_style);
            return;
        }

        let left_span = Span::styled(old_line[..col].to_string(), line_style);
        let cur_span = Span::styled(old_line[col..col + 1].to_string(), cur_style);
        let right_span = Span::styled(old_line[col + 1..].to_string(), line_style);

        lines[line] = Line::from(vec![left_span, cur_span, right_span]);
    }
}

impl StatefulWidget for TextWindow {
    type State = TextWindowState;

    fn render(self, area: Rect, tui_buf: &mut TUI_Buffer, state: &mut Self::State) {
        let line_numbers_width: u16 = (format!("{}", state.lines_count()).chars().count() + 1)
            .try_into()
            .expect("line number too large!");
        let window_layout = Layout::default()
            .direction(Direction::Horizontal)
            .constraints(vec![
                Constraint::Length(line_numbers_width),
                Constraint::Length(2),
                Constraint::Fill(1),
            ])
            .split(area);
        let theme = self.theme.upgrade().expect("referencing dropped theme!");
        let lines_area = window_layout[2];
        let mut lines = self.build_lines(lines_area.height, lines_area.width.into(), state);
        self.highlight_cursor(&mut lines, state);
        let line_numbers_area = window_layout[0];
        let line_hints_area = window_layout[1];
        let line_hints = Paragraph::new("").style(Style::default().bg(theme.text_background));
        let line_numbers = LineNumbers::new(
            Relative,
            state.top_line + 1,
            state.top_line + area.height as usize,
            state.cursor.line + 1,
        )
        .set_styles(
            theme.styles.line_numbers_normal,
            theme.styles.line_numbers_selected,
        );

        line_numbers.render(line_numbers_area, tui_buf);
        line_hints.render(line_hints_area, tui_buf);
        if lines.len() < lines_area.height as usize {
            let gap = lines_area.height - lines.len() as u16;
            let gap_area = Layout::default()
                .direction(Direction::Vertical)
                .constraints(vec![Constraint::Fill(1), Constraint::Length(gap)])
                .split(area)[1];
            Block::new()
                .bg(theme.text_background)
                .fg(theme.text_background)
                .render(gap_area, tui_buf);
        }
        Paragraph::new(lines).render(lines_area, tui_buf);
    }
}
