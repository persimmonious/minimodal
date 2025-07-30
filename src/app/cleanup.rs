use std::{io::stdout, process::exit};

use crossterm::{
    execute,
    terminal::{disable_raw_mode, LeaveAlternateScreen},
};

const GRACEFUL_EXIT_CODE: i32 = 1;

pub fn graceful_exit(msg: Option<&str>) -> ! {
    execute!(stdout(), LeaveAlternateScreen).expect("cleanup failed during graceful exit");
    disable_raw_mode().expect("cleanup failed during graceful exit");
    ratatui::restore();
    if let Some(msg) = msg {
        eprintln!("{msg}");
    }
    exit(GRACEFUL_EXIT_CODE);
}

pub trait CleanUnwrap<T> {
    fn clean_unwrap(opt: Option<T>) -> T;
    fn clean_expect(opt: Option<T>, msg: &str) -> T;
}

impl<T> CleanUnwrap<T> for Option<T> {
    fn clean_unwrap(opt: Self) -> T {
        match opt {
            Some(x) => x,
            None => {
                graceful_exit(None);
            }
        }
    }

    fn clean_expect(opt: Self, msg: &str) -> T {
        match opt {
            Some(x) => x,
            None => {
                graceful_exit(Some(msg));
            }
        }
    }
}
