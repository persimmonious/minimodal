use clap::{arg, command};
use std::error::Error;

#[derive(Debug)]
pub struct Config {
    pub file_name: Option<String>,
}

impl Config {
    fn new() -> Config {
        return Config { file_name: None };
    }
}

pub fn parse_command_line() -> Result<Config, Box<dyn Error>> {
    let mut config = Config::new();
    let arg_matches = command!()
        .arg(arg!(--file <FILENAME>).required(false))
        .get_matches();

    let file_name_arg = arg_matches.get_one::<String>("file");
    match file_name_arg {
        Some(file_name_str) => {
            config.file_name = Some(file_name_str.to_owned());
        }
        None => (),
    }
    return Ok(config);
}

