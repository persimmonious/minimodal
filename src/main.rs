#[allow(non_snake_case)]
pub mod app;
mod config;
#[cfg(test)]
mod tests;

use ratatui::{prelude::CrosstermBackend, Terminal};
use std::{error::Error, io::stdout};

fn main() -> Result<(), Box<dyn Error>> {
    let config = config::parse_command_line()?;
    let mut terminal = Terminal::new(CrosstermBackend::new(stdout()))?;
    let app_result = app::run(&mut terminal, config);
    ratatui::restore();
    match app_result {
        Err(error) => Err(Box::new(error)),
        Ok(()) => Ok(()),
    }
}
