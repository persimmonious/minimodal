use ratatui::style::{Color, Style};

#[derive(Debug)]
pub struct Theme {
    pub text_background: Color,
    pub text_foreground: Color,
    pub selected_line_background: Color,
    pub selected_line_foreground: Color,
    pub tabline_foreground: Color,
    pub tabline_background: Color,
    pub tabline_border: Color,
    
}

impl Default for Theme {
    fn default() -> Self {
        return Theme {
            text_background: Color::Rgb(10, 30, 30),
            text_foreground: Color::Rgb(240, 230, 220),
            selected_line_background: Color::Rgb(80, 80, 80),
            selected_line_foreground: Color::Rgb(240, 230, 220),
            tabline_foreground: Color::Rgb(144, 190, 255),
            tabline_background: Color::Rgb(20, 20, 40),
            tabline_border: Color::Rgb(80, 120, 180),
        }
    }
}
