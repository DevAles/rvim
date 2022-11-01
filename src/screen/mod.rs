use std::io::{stdout, Write};

use crossterm::{execute, queue, terminal, terminal::ClearType};

use crate::tools::cursor::CursorController;

const RVIM_VERSION: f64 = 0.1;

pub struct Buffer {
    content: String,
}

pub struct Screen {
    pub width: usize,
    pub height: usize,
    pub buffer: Buffer,
    pub cursor: CursorController,
}

impl Buffer {
    pub fn new() -> Self {
        Buffer {
            content: String::new(),
        }
    }

    pub fn push(&mut self, character: char) {
        self.content.push(character);
    }

    pub fn push_string(&mut self, string: &str) {
        self.content.push_str(string);
    }
}

impl std::io::Write for Buffer {
    fn write(&mut self, buffer: &[u8]) -> std::io::Result<usize> {
        match std::str::from_utf8(buffer) {
            Ok(string) => {
                self.push_string(string);
                Ok(string.len())
            }

            Err(_) => Err(std::io::ErrorKind::WriteZero.into()),
        }
    }

    fn flush(&mut self) -> std::io::Result<()> {
        let out = write!(stdout(), "{}", self.content);
        stdout().flush()?;
        self.content.clear();

        out
    }
}

impl Screen {
    pub fn new() -> crossterm::Result<Self> {
        Screen::clear()?;

        let (width, height) = terminal::size()
            .map(|(x, y)| (x as usize, y as usize))
            .unwrap();
        let buffer = Buffer::new();
        let cursor = CursorController::new();

        Ok(Screen {
            width,
            height,
            buffer,
            cursor,
        })
    }

    pub fn clear() -> crossterm::Result<()> {
        execute!(stdout(), terminal::Clear(ClearType::All))?; // In start, this will avoid compile messages in screen
        execute!(stdout(), terminal::Clear(ClearType::Purge))?;
        execute!(stdout(), crossterm::cursor::MoveTo(0, 0))
    }

    pub fn padding_message(&mut self, string: &str) {
        let mut padding = (self.width - string.len()) / 2;

        if padding != 0 {
            self.buffer.push('~');
            padding -= 1;
        }

        (0..padding).for_each(|_| {
            self.buffer.push(' ');
        });

        self.buffer.push_string(string)
    }

    pub fn render_version_message(&mut self) {
        let mut version_message = format!("> rvim version {}", RVIM_VERSION);

        if version_message.len() > self.width {
            version_message.truncate(self.width);
        }

        self.padding_message(&version_message);
    }

    pub fn create_rows(&mut self) {
        for i in 0..self.height {
            if i == self.height / 3 {
                self.render_version_message();
            } else {
                self.buffer.push('~');
            }

            queue!(self.buffer, terminal::Clear(ClearType::UntilNewLine)).unwrap();

            if i < self.height - 1 {
                self.buffer.push_string("\r\n");
            }
        }
    }

    pub fn refresh(&mut self) -> crossterm::Result<bool> {
        // In refresh, this ClearType::Purge will avoid print the escape key.
        queue!(
            self.buffer,
            crossterm::cursor::Hide,
            crossterm::cursor::MoveTo(self.cursor.x as u16, self.cursor.y as u16)
        )?;
        self.create_rows();

        queue!(
            self.buffer,
            crossterm::cursor::Show,
            crossterm::cursor::MoveTo(self.cursor.x as u16, self.cursor.x as u16)
        )?;
        self.buffer.flush()?;

        Ok(true)
    }
}
