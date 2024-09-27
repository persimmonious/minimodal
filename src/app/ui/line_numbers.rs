use ratatui::{
    buffer::Buffer as TUI_Buffer,
    layout::Rect,
    style::{Modifier, Style, Styled, Stylize},
    text::Line,
    widgets::{Paragraph, Widget},
};

pub enum LineNumberType {
    Absolute,
    Relative,
}

struct LineNumberStyles {
    normal: Style,
    selected: Style,
}

pub struct LineNumbers {
    number_type: LineNumberType,
    first_line: usize,
    last_line: usize,
    selected: usize,
    style: LineNumberStyles,
}

impl LineNumbers {
    pub fn new(
        number_type: LineNumberType,
        first_line: usize,
        last_line: usize,
        selected: usize,
    ) -> Self {
        return LineNumbers {
            number_type,
            first_line,
            last_line,
            selected,
            style: LineNumberStyles {
                normal: Style::default(),
                selected: Style::default().add_modifier(Modifier::BOLD),
            },
        };
    }

    pub fn generate_numbers(&self) -> Vec<usize> {
        let numbers: Vec<usize> = (self.first_line..self.last_line + 1)
            .map(|x| x as usize)
            .collect();
        match self.number_type {
            LineNumberType::Absolute => numbers,
            LineNumberType::Relative => numbers
                .iter()
                .map(|&x| {
                    if x == self.selected {
                        x
                    } else if x > self.selected as usize {
                        x - self.selected as usize
                    } else {
                        self.selected as usize - x
                    }
                })
                .collect(),
        }
    }

    pub fn set_styles(&mut self, normal: Style, selected: Style) {}
}

impl Widget for &LineNumbers {
    fn render(self, area: Rect, buf: &mut TUI_Buffer) {
        let numbers = &self.generate_numbers();
        let width = area.width as usize;
        let mut lines: Vec<Line> = numbers
            .iter()
            .map(|&x| Line::styled(format!("{x: >width$}"), self.style.normal))
            .collect();
        if self.selected >= self.first_line && self.selected <= self.last_line {
            let selected_index = self.selected - self.first_line;
            lines[selected_index] = lines[selected_index].clone().style(self.style.selected);
        }
        Paragraph::new(lines).render(area, buf);
    }
}
