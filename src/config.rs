use clap::{command, Arg, ArgAction};
use std::error::Error;

#[derive(Debug)]
pub struct Config {
    pub file_names: Vec<String>,
}

impl Config {
    fn new() -> Self {
        return Config { file_names: vec![] };
    }
}

pub fn parse_command_line() -> Result<Config, Box<dyn Error>> {
    let mut config = Config::new();
    let arg_matches = command!()
        .arg(Arg::new("files").action(ArgAction::Append))
        .get_matches();

    if let Some(file_names) = arg_matches.get_many::<String>("files") {
        config.file_names = file_names.map(|x| x.to_owned()).collect();
    }

    return Ok(config);
}
