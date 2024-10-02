use std::{
    ffi::{OsStr, OsString},
    fs, io,
};

#[derive(Debug)]
pub enum HorizontalDirection {
    Forwards,
    Backwards,
}

#[derive(Debug)]
pub enum VerticalDirection {
    Up,
    Down,
}

#[derive(Debug)]
pub enum RectilinearDirection {
    Up,
    Right,
    Down,
    Left,
}

#[derive(Debug)]
pub struct BufferPosition {
    pub line: usize,
    pub col: usize,
}

#[derive(Debug)]
pub struct Buffer {
    name: Option<OsString>,
    path: Option<OsString>,
    pub lines: Vec<String>,
}

impl Buffer {
    pub fn read_name<'a>(&'a self) -> Option<&'a OsStr> {
        match &self.name {
            Some(name) => Some(name),
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

    pub fn load(name: OsString, path: OsString) -> io::Result<Self> {
        let contents = fs::read_to_string(&path)?;
        Ok(Buffer {
            name: Some(name),
            path: Some(path),
            lines: contents.lines().map(|line| line.to_owned()).collect(),
        })
    }

    pub fn empty(name: OsString, path: OsString) -> Self {
        Buffer {
            name: Some(name),
            path: Some(path),
            lines: vec![],
        }
    }

    pub fn insert_char(&mut self, c: char, pos: &BufferPosition) {
        if self.lines.len() == 0 {
            self.lines.push(String::from(c));
            return;
        }
        let BufferPosition { line, col } = *pos;
        self.lines[line].insert(col, c);
    }
}
