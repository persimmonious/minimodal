use ratatui::style::{Color, Style};

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
}

impl Default for Theme {
    fn default() -> Self {
        return Theme {
            text_background: Color::Rgb(35, 35, 40),
            text_foreground: Color::Rgb(220, 200, 180),
            selected_line_background: Color::Rgb(45, 45, 50),
            selected_line_foreground: Color::Rgb(200, 200, 190),
            tabline_foreground: Color::Rgb(144, 190, 255),
            tabline_background: Color::Rgb(20, 20, 40),
            tabline_border_foreground: Color::Rgb(80, 120, 180),
            tabline_border_background: Color::Rgb(20, 20, 40),
        };
    }
}
