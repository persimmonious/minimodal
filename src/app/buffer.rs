use std::{
    ffi::{OsStr, OsString},
    fs::{self, File},
    io::{self, Write},
};

#[derive(Debug, Clone)]
pub enum HorizontalDirection {
    Forwards,
    Backwards,
}

#[derive(Debug, Clone)]
pub enum VerticalDirection {
    Up,
    Down,
}

#[derive(Debug, Clone)]
pub enum RectilinearDirection {
    Up,
    Right,
    Down,
    Left,
}

#[derive(Debug, Clone)]
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

    pub fn save(&self) -> io::Result<()> {
        let linebreak = "\n";
        let path = self.path.as_ref().unwrap();
        let mut file = io::LineWriter::new(File::create(path)?);
        for line in &self.lines {
            file.write(line.as_bytes())?;
            file.write(linebreak.as_bytes())?;
        }
        Ok(())
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

    pub fn clear_line(&mut self, pos: &BufferPosition) {
        if self.lines.len() == 0 {
            self.lines.push(String::new());
            return;
        }
        let line = pos.line;
        self.lines[line] = String::new();
    }

    pub fn add_line(&mut self, index: usize, content: String) {
        self.lines.insert(index, content);
    }

    pub fn split_line(&mut self, pos: &BufferPosition) {
        let BufferPosition { line, col } = *pos;
        let new_line: String = self.lines[line].chars().skip(col).collect();
        self.add_line(line + 1, new_line);
        let old_line: String = self.lines[line].chars().take(col).collect();
        self.lines[line] = old_line;
    }
}
