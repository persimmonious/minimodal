use std::{fmt::Debug, io::stdout, process::exit};

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
    fn clean_unwrap(self) -> T;
    fn clean_expect(self, msg: &str) -> T;
}

impl<T> CleanUnwrap<T> for Option<T> {
    fn clean_unwrap(self) -> T {
        match self {
            Some(x) => x,
            None => {
                graceful_exit(None);
            }
        }
    }

    fn clean_expect(self, msg: &str) -> T {
        match self {
            Some(x) => x,
            None => {
                graceful_exit(Some(msg));
            }
        }
    }
}

impl<T, E> CleanUnwrap<T> for Result<T, E>
where
    E: Debug,
{
    fn clean_unwrap(self) -> T {
        match self {
            Ok(x) => x,
            Err(e) => graceful_exit(Some(&format!("{e:?}"))),
        }
    }

    fn clean_expect(self, msg: &str) -> T {
        match self {
            Ok(x) => x,
            Err(e) => {
                graceful_exit(Some(&format!("{msg}\n{e:?}")));
            }
        }
    }
}
