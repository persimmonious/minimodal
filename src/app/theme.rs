use ratatui::style::{Color, Modifier, Style};

#[derive(Debug)]
pub struct Styles {
    pub line_numbers_normal: Style,
    pub line_numbers_selected: Style,
    pub status_mode_normal: Style,
    pub status_mode_command: Style,
    pub status_mode_insert: Style,
    pub status_mode_menu: Style,
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
    pub status_mode_normal_background: Color,
    pub status_mode_normal_foreground: Color,
    pub status_mode_command_background: Color,
    pub status_mode_command_foreground: Color,
    pub status_mode_insert_background: Color,
    pub status_mode_insert_foreground: Color,
    pub status_mode_menu_background: Color,
    pub status_mode_menu_foreground: Color,
    pub status_background: Color,
    pub status_foreground: Color,
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
        let status_mode_normal_background = tabline_foreground;
        let status_mode_normal_foreground = text_background;
        let status_mode_command_background = text_foreground;
        let status_mode_command_foreground = text_background;
        let status_mode_insert_background = Color::Rgb(120, 240, 140);
        let status_mode_insert_foreground = text_background;
        let status_mode_menu_background = Color::Rgb(220, 240, 140);
        let status_mode_menu_foreground = text_background;
        let status_background = Color::Rgb(10, 10, 10);
        let status_foreground = text_foreground;

        let line_numbers_normal = Style::default()
            .fg(line_numbers_normal_foreground)
            .bg(line_numbers_normal_background);
        let line_numbers_selected = Style::default()
            .fg(line_numbers_selected_foreground)
            .bg(line_numbers_selected_background)
            .add_modifier(Modifier::BOLD);
        let status_mode_normal = Style::default()
            .fg(status_mode_normal_foreground)
            .bg(status_mode_normal_background);
        let status_mode_command = Style::default()
            .fg(status_mode_command_foreground)
            .bg(status_mode_command_background);
        let status_mode_insert = Style::default()
            .fg(status_mode_insert_foreground)
            .bg(status_mode_insert_background);
        let status_mode_menu = Style::default()
            .fg(status_mode_menu_foreground)
            .bg(status_mode_menu_background);

        let styles = Styles {
            line_numbers_normal,
            line_numbers_selected,
            status_mode_normal,
            status_mode_command,
            status_mode_insert,
            status_mode_menu,
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
            status_mode_normal_background,
            status_mode_normal_foreground,
            status_mode_command_background,
            status_mode_command_foreground,
            status_mode_insert_background,
            status_mode_insert_foreground,
            status_mode_menu_background,
            status_mode_menu_foreground,
            status_background,
            status_foreground,
            styles,
        };
    }
}
