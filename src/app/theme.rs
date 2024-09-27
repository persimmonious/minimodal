use ratatui::style::{Color, Modifier, Style};

#[derive(Debug)]
pub struct Styles {
    pub line_numbers_normal: Style,
    pub line_numbers_selected: Style,
}

#[derive(Debug)]
pub struct Theme {
    pub text_background: Color,
    pub text_foreground: Color,
    pub selected_line_background: Color,
    pub selected_line_foreground: Color,
    pub tabline_foreground: Color,
    pub tabline_background: Color,
    pub tabline_border_foreground: Color,
    pub tabline_border_background: Color,
    pub line_numbers_normal_foreground: Color,
    pub line_numbers_normal_background: Color,
    pub line_numbers_selected_foreground: Color,
    pub line_numbers_selected_background: Color,
    pub styles: Styles,
}

impl Default for Theme {
    fn default() -> Self {
        let text_background = Color::Rgb(35, 35, 40);
        let text_foreground = Color::Rgb(220, 200, 180);
        let selected_line_background = Color::Rgb(50, 50, 55);
        let selected_line_foreground = text_foreground;
        let tabline_foreground = Color::Rgb(144, 190, 255);
        let tabline_background = Color::Rgb(20, 20, 40);
        let tabline_border_foreground = Color::Rgb(80, 120, 180);
        let tabline_border_background = Color::Rgb(20, 20, 40);
        let line_numbers_normal_foreground = Color::Rgb(90, 90, 90);
        let line_numbers_normal_background = text_background;
        let line_numbers_selected_foreground = text_foreground;
        let line_numbers_selected_background = text_background;

        let line_numbers_normal = Style::default()
            .fg(line_numbers_normal_foreground)
            .bg(line_numbers_normal_background);
        let line_numbers_selected = Style::default()
            .fg(line_numbers_selected_foreground)
            .bg(line_numbers_selected_background)
            .add_modifier(Modifier::BOLD);

        let styles = Styles {
            line_numbers_normal,
            line_numbers_selected,
        };

        return Theme {
            text_background,
            text_foreground,
            selected_line_background,
            selected_line_foreground,
            tabline_foreground,
            tabline_background,
            tabline_border_foreground,
            tabline_border_background,
            line_numbers_normal_foreground,
            line_numbers_normal_background,
            line_numbers_selected_foreground,
            line_numbers_selected_background,
            styles,
        };
    }
}
