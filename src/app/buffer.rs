use std::{fs, io};

#[derive(Debug)]
pub struct Buffer {
    name: Option<String>,
    path: Option<String>,
    pub lines: Vec<String>,
}

impl Buffer {
    pub fn read_name<'a>(&'a self) -> Option<&'a str> {
        match &self.name {
            Some(name) => Some(&name),
            None => None,
        }
    }

    pub fn untitled() -> Self {
        return Buffer {
            name: None,
            path: None,
            lines: vec![],
        };
    }

    pub fn load(name: &str, path: &str) -> io::Result<Self> {
        let contents = fs::read_to_string(path)?;
        Ok(Buffer {
            name: Some(name.to_owned()),
            path: Some(path.to_owned()),
            lines: contents.lines().map(|line| line.to_owned()).collect(),
        })
    }

    pub fn empty(name: &str, path: &str) -> Self {
        Buffer {
            name: Some(name.to_owned()),
            path: Some(path.to_owned()),
            lines: vec![],
        }
    }
}
