use std::cmp::max;

use ratatui::{
    layout::{Constraint, Flex, Layout},
    prelude::{Buffer as TUI_Buffer, Rect},
    style::{Color, Style, Stylize},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph, Widget},
};
use SubMenu::*;

use crate::app::theme::Theme;

pub const KEY_HINT_SEPARATOR: &'static str = " : ";
const MINIMUM_COLUMN_SPACING: u16 = 2;

#[derive(Debug, Clone, PartialEq)]
pub enum SubMenu {
    Root,
}

#[derive(Debug)]
pub struct LeaderMenu {
    sub_menu: SubMenu,
    menu_background: Color,
    menu_border: Color,
    key_hint_style: KeyHintStyle,
}

#[derive(Debug)]
pub struct KeyHintStyle {
    background: Color,
    key: Color,
    separator: Color,
    action: Color,
}

#[derive(Debug)]
pub struct KeyHint {
    key: String,
    action: String,
}

impl KeyHint {
    pub fn new(key: &str, action: &str) -> Self {
        return KeyHint {
            key: key.to_string(),
            action: action.to_string(),
        };
    }

    pub fn styled<'a>(self, style: &KeyHintStyle) -> Line<'a> {
        let base = Style::default().bg(style.background);
        let key = Span::styled(self.key.clone(), base.fg(style.key));
        let sep = Span::styled(KEY_HINT_SEPARATOR, base.fg(style.separator));
        let action = Span::styled(self.action.clone(), base.fg(style.action));
        return Line::from(vec![key, sep, action]);
    }

    pub fn len(&self) -> usize {
        return self.key.len() + self.action.len() + KEY_HINT_SEPARATOR.len();
    }
}

impl LeaderMenu {
    pub fn new(sub_menu: &SubMenu, theme: &Theme) -> Self {
        return LeaderMenu {
            sub_menu: sub_menu.clone(),
            menu_background: theme.menu_background,
            menu_border: theme.menu_border,
            key_hint_style: KeyHintStyle {
                background: theme.menu_background,
                key: theme.menu_key_foreground,
                separator: theme.menu_separator_foreground,
                action: theme.menu_action_foreground,
            },
        };
    }

    pub fn required_height(sub_menu: &SubMenu, width: u16) -> u16 {
        let items = Self::menu_items(sub_menu);

        let mut height: usize = 1;
        loop {
            let mut col_widths: Vec<u16> = vec![];
            let mut col_count = 0;
            while let Some(col) = items.get(height * col_count..height * (col_count + 1)) {
                col_widths.push(
                    col.iter()
                        .map(|kh| kh.len())
                        .fold(0, |acc, n| max(acc, n as u16)),
                );
                col_count += 1;
            }
            let new_width =
                (col_count + 1) as u16 * MINIMUM_COLUMN_SPACING + col_widths.iter().sum::<u16>();
            if new_width <= width {
                break;
            }
            height += 1;
        }
        return (height + 3) as u16;
    }

    fn style_keyhints<'a>(hints: Box<Vec<KeyHint>>, style: &KeyHintStyle) -> Box<Vec<Line<'a>>> {
        return Box::new(hints.into_iter().map(move |h| h.styled(style)).collect());
    }

    pub fn menu_items(sub_menu: &SubMenu) -> Box<Vec<KeyHint>> {
        return match sub_menu {
            Root => Self::root_menu_items(),
        };
    }

    fn root_menu_items() -> Box<Vec<KeyHint>> {
        return Box::new(vec![
            KeyHint::new("q", "Quit"),
            KeyHint::new("w", "Save Buffer"),
        ]);
    }
}

impl Widget for LeaderMenu {
    fn render(self, area: Rect, buf: &mut TUI_Buffer) {
        let outer_layout = Layout::vertical(vec![
            Constraint::Length(1),
            Constraint::Fill(1),
            Constraint::Length(1),
            Constraint::Length(1),
        ])
        .split(area);
        Block::new()
            .fg(self.menu_background)
            .bg(self.menu_background)
            .borders(Borders::TOP)
            .border_type(ratatui::widgets::BorderType::QuadrantOutside)
            .border_style(self.menu_border)
            .render(area, buf);

        let height = outer_layout[1].height as usize;
        if height < 1 {
            return;
        }
        let width = area.width;

        let items = Self::style_keyhints(Self::menu_items(&self.sub_menu), &self.key_hint_style);
        let mut columns: Vec<Paragraph> = vec![];
        let mut minimum_width = MINIMUM_COLUMN_SPACING;
        while let Some(col) = items.get(height * columns.len()..height * (columns.len() + 1)) {
            let column: Vec<_> = col.iter().cloned().collect();
            let column = Paragraph::new(column);
            minimum_width += column.line_width() as u16 + MINIMUM_COLUMN_SPACING;
            if minimum_width > width {
                break;
            }
            columns.push(column);
        }

        let constraints: Vec<Constraint> = columns
            .iter()
            .map(|col| Constraint::Length(col.line_width() as u16))
            .collect();

        Layout::horizontal(constraints)
            .flex(Flex::SpaceAround)
            .split(outer_layout[1])
            .iter()
            .zip(columns)
            .for_each(|(&space, text)| text.render(space, buf));
    }
}
