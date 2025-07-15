use super::line_numbers::LineNumberType::Relative;
use super::line_numbers::LineNumbers;
use crate::app::{
    buffer::{Buffer, BufferPosition, RectilinearDirection as Rectilinear},
    editor::Mode,
    theme::Theme,
};
use ratatui::{
    buffer::Buffer as TUI_Buffer,
    layout::{Constraint, Direction, Layout, Position, Rect},
    style::{Style, Stylize},
    text::{Line, Span},
    widgets::{Block, Paragraph, StatefulWidget, Widget},
};
use std::{cell::RefCell, iter::repeat_n};
use std::{
    cmp::{max, min},
    rc::Weak,
};

#[derive(Debug, Clone)]
pub struct TextWindow {
    buffer: Weak<RefCell<Buffer>>,
    theme: Weak<Theme>,
}

#[derive(Debug, Clone)]
pub struct ScreenBounds {
    top_line: usize,
    bottom_line: usize,
    leftmost_col: usize,
    rightmost_col: usize,
}

#[derive(Debug)]
pub struct TextWindowState {
    pub top_line: usize,
    pub leftmost_col: usize,
    pub last_height: usize,
    pub last_width: usize,
    pub cur_vertical_percent: f32,
    pub cursor: BufferPosition,
    pub last_manual_col: usize,
    pub stick_to_EOL: bool,
    buffer: Weak<RefCell<Buffer>>,
    theme: Weak<Theme>,
}

impl TextWindowState {
    pub fn new(buffer: Weak<RefCell<Buffer>>, theme: Weak<Theme>) -> Self {
        TextWindowState {
            top_line: 0,
            leftmost_col: 0,
            last_height: 2,
            last_width: 2,
            cur_vertical_percent: 0.0,
            cursor: BufferPosition { line: 0, col: 0 },
            last_manual_col: 0,
            stick_to_EOL: false,
            buffer,
            theme,
        }
    }

    pub fn move_cursor(&mut self, mode: &Mode, dir: Rectilinear) {
        match (mode, dir) {
            (Mode::Normal | Mode::Insert, Rectilinear::Up) => {
                if self.cursor.line == 0 {
                    return;
                }
                let mut relative_line = self.cursor.line - self.top_line;
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
                if self.cursor.col >= new_line_length || self.stick_to_EOL {
                    self.jump_to_EOL();
                    if matches!(mode, Mode::Insert) {
                        let last_manual_column = self.last_manual_col;
                        self.jump_past_EOL();
                        self.last_manual_col = last_manual_column;
                    }
                } else if matches!(mode, Mode::Insert) && self.last_manual_col >= new_line_length {
                    if self.last_manual_col > new_line_length {
                        let last_manual_column = self.last_manual_col;
                        self.jump_past_EOL();
                        self.last_manual_col = last_manual_column;
                    } else {
                        self.jump_past_EOL();
                    }
                } else {
                    self.jump(&BufferPosition {
                        line: self.cursor.line,
                        col: min(self.last_manual_col, max(new_line_length, 1) - 1),
                    });
                }
            }

            (Mode::Normal | Mode::Insert, Rectilinear::Down) => {
                if self.cursor.line + 1 >= self.lines_count() {
                    return;
                }
                let mut relative_line = self.cursor.line - self.top_line;

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
                if self.cursor.col >= new_line_length || self.stick_to_EOL {
                    self.jump_to_EOL();
                    if matches!(mode, Mode::Insert) {
                        let last_manual_column = self.last_manual_col;
                        self.jump_past_EOL();
                        self.last_manual_col = last_manual_column;
                    }
                } else if matches!(mode, Mode::Insert) && self.last_manual_col >= new_line_length {
                    if self.last_manual_col > new_line_length {
                        let last_manual_column = self.last_manual_col;
                        self.jump_past_EOL();
                        self.last_manual_col = last_manual_column;
                    } else {
                        self.jump_past_EOL();
                    }
                } else {
                    self.jump(&BufferPosition {
                        line: self.cursor.line,
                        col: min(self.last_manual_col, max(new_line_length, 1) - 1),
                    });
                }
            }

            (Mode::Normal, Rectilinear::Right) => {
                if self.lines_count() == 0 {
                    return;
                }
                let line_length = self.line_length(self.cursor.line);
                if self.cursor.col + 1 >= line_length {
                    return;
                }
                self.stick_to_EOL = false;
                self.cursor.col += 1;
                self.last_manual_col = self.cursor.col;
                if self.cursor.col >= self.leftmost_col + self.last_width {
                    self.leftmost_col += 1;
                }
            }

            (Mode::Insert, Rectilinear::Right) => {
                if self.lines_count() == 0 {
                    return;
                }
                let line = self.cursor.line;
                if self.cursor_past_EOL() {
                    if line == self.lines_count() - 1 {
                        return;
                    }
                    self.stick_to_EOL = false;
                    self.jump(&BufferPosition {
                        line: line + 1,
                        col: 0,
                    });
                } else {
                    self.stick_to_EOL = false;
                    self.cursor.col += 1;
                    self.last_manual_col = self.cursor.col;
                    if self.cursor.col >= self.leftmost_col + self.last_width {
                        self.leftmost_col += 1;
                    }
                }
            }

            (Mode::Normal, Rectilinear::Left) => {
                if self.cursor.col == 0 {
                    return;
                }
                self.stick_to_EOL = false;
                self.cursor.col -= 1;
                self.last_manual_col = self.cursor.col;
                if self.cursor.col < self.leftmost_col {
                    self.leftmost_col = self.cursor.col;
                }
            }

            (Mode::Insert, Rectilinear::Left) => {
                if self.lines_count() == 0 {
                    return;
                }
                let line = self.cursor.line;
                if self.cursor.col == 0 {
                    if line == 0 {
                        return;
                    }
                    self.stick_to_EOL = false;
                    self.jump(&BufferPosition {
                        line: line - 1,
                        col: self.line_length(line - 1),
                    });
                    self.jump_past_EOL();
                    self.last_manual_col = self.cursor.col;
                } else {
                    self.stick_to_EOL = false;
                    self.cursor.col -= 1;
                    self.last_manual_col = self.cursor.col;
                    if self.cursor.col < self.leftmost_col {
                        self.leftmost_col = self.cursor.col;
                    }
                }
            }

            (Mode::Command | Mode::Menu(_), _) => (),
        }
    }

    pub fn advance_insertion_cursor(&mut self) {
        if self.lines_count() == 0 {
            return;
        }
        self.cursor.col += 1;
        self.last_manual_col = self.cursor.col;
        if self.cursor.col >= self.leftmost_col + self.last_width {
            self.leftmost_col += 1;
        }
    }

    fn screen_bounds(&self) -> ScreenBounds {
        let top_line = self.top_line;
        let bottom_line = top_line + self.last_height - 1;
        let leftmost_col = self.leftmost_col;
        let rightmost_col = leftmost_col + self.last_width - 1;
        ScreenBounds {
            top_line,
            bottom_line,
            leftmost_col,
            rightmost_col,
        }
    }

    fn is_on_screen(&self, pos: &BufferPosition) -> bool {
        let ScreenBounds {
            top_line,
            bottom_line,
            leftmost_col,
            rightmost_col,
        } = self.screen_bounds();
        let BufferPosition { line, col } = *pos;
        let within_vertically = line >= top_line && line <= bottom_line;
        let within_horizontally = col >= leftmost_col && col <= rightmost_col;
        within_vertically && within_horizontally
    }

    /// Checks if the cursor is past the last character in the line
    pub fn cursor_past_EOL(&self) -> bool {
        let BufferPosition { line, col } = self.cursor;
        if self.lines_count() == 0 {
            return true;
        }
        col >= self.line_length(line)
    }

    pub fn snap_to_EOL(&mut self) {
        if self.lines_count() == 0 {
            self.cursor.col = 0;
            self.leftmost_col = 0;
            return;
        }
        let line_length = self.line_length(self.cursor.line);
        if self.cursor.col >= line_length {
            self.jump_to_EOL();
        }
    }

    fn jump_within_screen(&mut self, pos: &BufferPosition) {
        self.cursor.line = pos.line;
        self.cursor.col = pos.col;
        let relative_line = pos.line - self.top_line;
        self.cur_vertical_percent = relative_line as f32 / (self.last_height - 1) as f32;
    }

    pub fn jump(&mut self, pos: &BufferPosition) {
        if self.is_on_screen(pos) {
            self.jump_within_screen(pos);
            return;
        }
        let BufferPosition { line, col } = *pos;
        let ScreenBounds {
            top_line,
            bottom_line,
            leftmost_col,
            rightmost_col,
        } = self.screen_bounds();

        let vertically_out_of_bounds = line < top_line || line > bottom_line;
        if self.lines_count() > 0 && vertically_out_of_bounds {
            let line = min(line, self.lines_count() - 1);
            let relative_line = min(self.last_height / 2, line);
            self.cur_vertical_percent = relative_line as f32 / (self.last_height - 1) as f32;
        }

        if col < leftmost_col || col > rightmost_col {
            let relative_col = min(self.last_width * 3 / 4, col);
            self.leftmost_col = col - relative_col;
        }

        self.cursor = pos.to_owned();
        self.last_manual_col = self.cursor.col;
    }

    pub fn sticky_jump_to_EOL(&mut self) {
        self.stick_to_EOL = true;
        self.jump_to_EOL();
    }

    pub fn jump_to_EOL(&mut self) {
        if self.lines_count() == 0 {
            self.jump_to_home();
            return;
        }
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
        if to_the_right || self.cursor.col >= self.last_width {
            self.leftmost_col = self.cursor.col + 1 - self.last_width;
        } else {
            self.leftmost_col = 0;
        }
    }

    /// Jumps to a space immediately following the last character in the line.
    /// Meant for use in insert mode.
    pub fn jump_past_EOL(&mut self) {
        self.jump_to_EOL();
        if !self.cursor_past_EOL() {
            self.advance_insertion_cursor();
        }
    }

    pub fn jump_to_home(&mut self) {
        self.cursor.col = 0;
        self.leftmost_col = 0;
        self.last_manual_col = 0;
        self.stick_to_EOL = false;
    }

    pub fn jump_to_last_line(&mut self) {
        let line = if self.lines_count() > 0 {
            self.lines_count() - 1
        } else {
            0
        };
        self.top_line = if line >= self.last_height {
            line - self.last_height + 1
        } else {
            0
        };
        self.cursor.line = line;
        let relative_line = line - self.top_line;
        self.cur_vertical_percent = relative_line as f32 / (self.last_height - 1) as f32;
        self.snap_to_EOL();
        self.last_manual_col = self.cursor.col;
    }

    pub fn lines_count(&self) -> usize {
        self.buffer
            .upgrade()
            .expect("counting lines in a dead buffer!")
            .borrow()
            .lines
            .len()
    }

    fn line_length(&self, line: usize) -> usize {
        self.buffer
            .upgrade()
            .expect("checking line length in a dead buffer!")
            .borrow()
            .lines[line]
            .len()
    }

    pub fn get_cursor_pos(&self) -> Position {
        let BufferPosition { line, col } = self.cursor;
        let line_numbers_width = format!("{}", self.lines_count()).chars().count() + 1;
        if self.is_on_screen(&self.cursor) {
            Position {
                x: (col - self.leftmost_col + line_numbers_width + 2) as u16,
                y: (line - self.top_line) as u16,
            }
        } else {
            Position { x: 0, y: 0 }
        }
    }
}

impl TextWindow {
    pub fn new(buffer: Weak<RefCell<Buffer>>, theme: Weak<Theme>) -> TextWindow {
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
            (state.cur_vertical_percent * (height - 1) as f32).round() as usize;
        let top_line: usize = state.cursor.line.saturating_sub(cursor_rel_line);
        let last_line: usize = min(top_line + height as usize, state.lines_count());
        let line_style = Style::default()
            .fg(theme.text_foreground)
            .bg(theme.text_background);
        return buffer.borrow().lines[top_line..last_line]
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
            lines.push(Line::from(String::from_iter(repeat_n(
                " ",
                state.last_width - 1,
            ))));
        }

        if state.cursor.line < state.top_line {
            return;
        }
        let line = state.cursor.line - state.top_line;
        if line >= lines.len() {
            return;
        }

        let theme = self.theme.upgrade().expect("referencing dropped theme!");
        let line_style = Style::default()
            .bg(theme.selected_line_background)
            .fg(theme.selected_line_foreground);

        let old_line: String = lines[line].to_owned().into();
        if old_line.is_empty() {
            lines[line] = Line::styled(" ", line_style);
            return;
        }
        lines[line] = Line::from(Span::styled(old_line, line_style));
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
