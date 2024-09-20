mod app;
use std::io;

fn main() -> io::Result<()> {
    let mut terminal = ratatui::init();
    terminal.clear()?;
    let app_result = app::run(&mut terminal);
    ratatui::restore();
    app_result
}
