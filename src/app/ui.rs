use super::{buffer::Buffer, BufferPosition};
use ratatui::{
    buffer::Buffer as TUI_Buffer,
    layout::Rect,
    style::{Color, Modifier, Style, Stylize},
    text::{Line, Span},
    widgets::{Paragraph, Widget},
};
use std::{
    cmp::min,
    rc::{Rc, Weak},
};

#[derive(Debug)]
pub struct Tab {
    pub buffer: Rc<Buffer>,
    windows: TextWindow,
    current_window: usize,
}

impl Tab {
    pub fn new(buf: Buffer) -> Self {
        let buf_rc = Rc::new(buf);
        Tab {
            buffer: Rc::clone(&buf_rc),
            windows: TextWindow::new(Rc::downgrade(&buf_rc)),
            current_window: 0,
        }
    }
}

impl Widget for &Tab {
    fn render(self, area: Rect, buf: &mut TUI_Buffer) {
        self.windows.render(area, buf);
    }
}

#[derive(Debug)]
pub struct TextWindow {
    top_line: usize,
    buffer: Weak<Buffer>,
    cursor: BufferPosition,
}

impl Widget for &TextWindow {
    fn render(self, area: Rect, tui_buf: &mut TUI_Buffer) {
        let mut lines = self.build_lines(area.height.into());
        self.highlight_cursor(&mut lines);
        Paragraph::new(lines).render(area, tui_buf);
    }
}

impl TextWindow {
    pub fn new(buf_rc: Weak<Buffer>) -> TextWindow {
        TextWindow {
            top_line: 0,
            buffer: buf_rc,
            cursor: BufferPosition { line: 0, col: 0 },
        }
    }

    fn build_lines(&self, height: usize) -> Vec<Line> {
        let buffer = self
            .buffer
            .upgrade()
            .expect("building lines from a dead buffer!");

        let last_line = min(self.top_line + height, buffer.lines.len());
        return buffer.lines[self.top_line..last_line]
            .iter()
            .cloned()
            .map(|line| Line::from(line))
            .collect();
    }

    fn highlight_cursor(&self, lines: &mut Vec<Line>) {
        if self.cursor.line < self.top_line {
            return;
        }
        let line = self.cursor.line - self.top_line;
        if line >= lines.len() {
            return;
        }
        let col = self.cursor.col;

        let line_style = Style::default().bg(Color::Rgb(80, 80, 80));
        let cur_style = line_style.add_modifier(Modifier::REVERSED);

        let old_line: String = lines[line].to_owned().into();
        let left_span = Span::styled(old_line[..col].to_string(), line_style);
        let cur_span = Span::styled(old_line[col..col + 1].to_string(), cur_style);
        let right_span = Span::styled(old_line[col + 1..].to_string(), line_style);

        lines[line] = Line::from(vec![left_span, cur_span, right_span]);
    }
}
