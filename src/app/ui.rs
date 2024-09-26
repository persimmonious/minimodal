use super::buffer::{Buffer, BufferPosition, RectilinearDirection as Rectilinear};
use ratatui::{
    buffer::Buffer as TUI_Buffer,
    layout::Rect,
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Paragraph, StatefulWidget, Widget},
};
use std::{cmp::min, rc::{Rc, Weak}};

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
    pub fn new(buf: Buffer) -> Self {
        let buf_rc = Rc::new(buf);
        return TabState {
            buffer: Rc::clone(&buf_rc),
            window_states: TextWindowState::new(Rc::downgrade(&buf_rc)),
            windows: TextWindow::new(Rc::downgrade(&buf_rc)),
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
}

#[derive(Debug)]
pub struct TextWindowState {
    pub top_line: usize,
    pub cur_position: f32,
    pub cursor: BufferPosition,
    buffer: Weak<Buffer>,
}

impl TextWindowState {
    pub fn new(buffer: Weak<Buffer>) -> Self {
        return TextWindowState {
            top_line: 0,
            cur_position: 0.0,
            cursor: BufferPosition { line: 0, col: 0 },
            buffer,
        };
    }

    pub fn move_cursor(&mut self, dir: Rectilinear) {
        match dir {
            Rectilinear::Up => {
                if self.cursor.line > 0 {
                    self.cursor.line -= 1;
                }
            }
            Rectilinear::Down => {
                if self.cursor.line + 1 < self.lines_count() {
                    self.cursor.line += 1;
                }
            }
            Rectilinear::Right => {
                let line_length = self.line_length(self.cursor.line);
                if self.cursor.col + 1 < line_length {
                    self.cursor.col += 1;
                }
            }
            Rectilinear::Left => {
                if self.cursor.col > 0 {
                    self.cursor.col -= 1;
                }
            }
        }
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
    pub fn new(buf_rc: Weak<Buffer>) -> TextWindow {
        TextWindow { buffer: buf_rc }
    }

    fn build_lines(&self, height: u16, width: usize, state: &mut TextWindowState) -> Vec<Line> {
        let buffer = self
            .buffer
            .upgrade()
            .expect("building lines from a dead buffer!");

        let cursor_rel_line: usize = (state.cur_position * height as f32).floor() as usize;
        let mut top_line: usize = state.cursor.line;
        if cursor_rel_line > state.cursor.line {
            top_line = 0;
            state.cur_position = state.cursor.line as f32 / height as f32;
        }

        let last_line: usize = min(top_line + height as usize, state.lines_count());
        return buffer.lines[top_line..last_line]
            .iter()
            .cloned()
            .map(|line| format!("{line: <width$}"))
            .map(|line| Line::from(line))
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
        let col = state.cursor.col;

        let line_style = Style::default().bg(Color::Rgb(80, 80, 80));
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
        let mut lines = self.build_lines(area.height, area.width.into(), state);
        self.highlight_cursor(&mut lines, state);
        Paragraph::new(lines).render(area, tui_buf);
    }
}
