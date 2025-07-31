use std::{
    env,
    ffi::{OsStr, OsString},
    fs::{self, File},
    io::{self, Write},
    path::Path,
};

use super::cleanup::CleanUnwrap;

#[derive(Debug, Clone)]
pub enum HorizontalDirection {
    Forward,
    Backward,
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

#[derive(Debug, Clone, PartialEq)]
pub struct BufferPosition {
    pub line: usize,
    pub col: usize,
}

#[derive(Debug, PartialEq)]
pub struct Buffer {
    name: Option<OsString>,
    path: Option<OsString>,
    pub lines: Vec<String>,
}

impl Buffer {
    pub fn read_name(&self) -> Option<&OsStr> {
        match &self.name {
            Some(name) => Some(name),
            None => None,
        }
    }

    pub fn set_name(&mut self, new_name: OsString) {
        self.name = Some(new_name);
    }

    pub fn path(&self) -> Option<&OsStr> {
        match &self.path {
            Some(path) => Some(path),
            None => None,
        }
    }

    pub fn set_path(&mut self, new_path: OsString) {
        let path = Path::new(&new_path);
        let abs_path = if path.is_relative() {
            let cwd = env::current_dir().clean_unwrap();
            &cwd.join(path)
        } else {
            path
        };
        self.path = Some(abs_path.as_os_str().to_owned());
    }

    pub fn untitled() -> Self {
        Buffer {
            name: None,
            path: None,
            lines: vec![],
        }
    }

    pub fn save(&self) -> io::Result<()> {
        let linebreak = "\n";
        let path = self.path.as_ref().clean_unwrap();
        let mut file = io::LineWriter::new(File::create(path)?);
        for line in &self.lines {
            file.write_all(line.as_bytes())?;
            file.write_all(linebreak.as_bytes())?;
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

    pub fn lines_count(&self) -> usize {
        self.lines.len()
    }

    pub fn line_length(&self, index: usize) -> Option<usize> {
        let count = self.lines_count();
        if count == 0 || index >= count {
            return None;
        }
        Some(self.lines[index].chars().count())
    }

    pub fn insert_char(&mut self, c: char, pos: &BufferPosition) {
        if self.lines.is_empty() {
            self.lines.push(String::from(c));
            return;
        }
        let BufferPosition { line, col } = *pos;
        let mut new_line: Vec<_> = self.lines[line].chars().collect();
        new_line.insert(col, c);
        self.lines[line] = String::from_iter(new_line);
    }

    pub fn remove_char(&mut self, pos: &BufferPosition) {
        if self.lines.is_empty() {
            return;
        }
        let BufferPosition { line, col } = *pos;
        if line < self.lines.len() {
            let len = self.lines[line].chars().count();
            if col >= len {
                return;
            }
            let new_line = self.lines[line]
                .chars()
                .enumerate()
                .flat_map(|(i, c)| if i != col { Some(c) } else { None })
                .collect();
            self.lines[line] = new_line;
        }
    }

    pub fn clear_line(&mut self, pos: &BufferPosition) {
        if self.lines.is_empty() {
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
        if self.lines.is_empty() {
            self.lines.push(String::new());
        }
        let BufferPosition { line, col } = *pos;
        let new_line: String = self.lines[line].chars().skip(col).collect();
        self.add_line(line + 1, new_line);
        let old_line: String = self.lines[line].chars().take(col).collect();
        self.lines[line] = old_line;
    }

    pub fn join_with_next_line(&mut self, line: usize) {
        if line + 1 >= self.lines.len() {
            return;
        }
        let second_line = self.lines[line + 1].to_owned();
        self.lines[line].push_str(&second_line);
        self.lines.remove(line + 1);
    }
}
