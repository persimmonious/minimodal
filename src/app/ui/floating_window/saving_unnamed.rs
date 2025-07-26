use std::{ffi::OsString, rc::Rc};

use crossterm::event::{KeyCode, KeyEvent};
use ratatui::{
    layout::{Constraint, Layout, Margin, Rect},
    style::{Style, Stylize},
    text::Line,
    widgets::{Block, BorderType, Borders},
    Frame,
};
use tui_textarea::TextArea;

use crate::app::{
    editor::{actions::EditorAction, Editor},
    theme::Theme,
};

use super::{EditorCallback, FloatingContent};

#[derive(Clone, Default)]
pub(crate) struct SavingUnnamed<'a> {
    filename: TextArea<'a>,
}

impl<'b> SavingUnnamed<'b> {}

impl<'b> FloatingContent for SavingUnnamed<'b> {
    fn handle_input(&mut self, input: &KeyEvent) -> Option<EditorCallback> {
        match input.code {
            KeyCode::Enter => {
                let new_name: OsString = self.filename.lines()[0].to_owned().into();
                Some(Box::new(|ed: &mut Editor| {
                    ed.current_buffer_mut().set_name(new_name.clone());
                    ed.current_buffer_mut().set_path(new_name);
                    ed.execute_editor_action(EditorAction::SaveBuffer);
                    ed.clear_floating_window()
                }))
            }
            KeyCode::Esc => Some(Box::new(|ed: &mut Editor| ed.clear_floating_window())),
            KeyCode::Char(_) | KeyCode::Backspace | KeyCode::Delete => {
                self.filename.input(*input);
                None
            }
            _ => None,
        }
    }

    fn render(&self, area: &Rect, frame: &mut Frame, theme: Rc<Theme>) {
        if area.height < 6 {
            return;
        }
        let outer_layout = Layout::vertical([
            Constraint::Fill(1),
            Constraint::Length(6),
            Constraint::Fill(1),
        ]);
        let window_area = outer_layout.split(*area)[1];
        let background = Block::new()
            .borders(Borders::all())
            .border_type(BorderType::Rounded)
            .bg(theme.menu_background)
            .fg(theme.text_foreground);
        frame.render_widget(background, window_area.to_owned());
        let inner_area = window_area.inner(Margin::new(2, 2));
        let inner_layout =
            Layout::vertical([Constraint::Length(1), Constraint::Length(1)]).split(inner_area);
        let message_area = inner_layout[0];
        let input_area = inner_layout[1];
        frame.render_widget(Line::from("Save the buffer as:"), message_area);
        let mut filename = self.filename.to_owned();
        filename.set_style(
            Style::default()
                .bg(theme.menu_background)
                .fg(theme.menu_key_foreground),
        );
        frame.render_widget(&self.filename, input_area);
    }

    fn clone_as_box(&self) -> Box<dyn FloatingContent> {
        let mut filename = TextArea::new(self.filename.lines().to_vec());
        filename.set_style(self.filename.style());
        Box::new(SavingUnnamed { filename })
    }
}
