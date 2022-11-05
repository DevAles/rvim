use crossterm::event::{self, Event, KeyCode, KeyEvent};

use crate::screen;

const MOVE_CURSOR_KEYS: [KeyCode; 8] = [
    KeyCode::Up,
    KeyCode::Down,
    KeyCode::Left,
    KeyCode::Right,
    KeyCode::PageUp,
    KeyCode::PageDown,
    KeyCode::Home,
    KeyCode::End,
];

pub struct Cursor {
    pub x: usize,
    pub y: usize,
}

pub struct Keyboard {
    pub cursor: Cursor,
}

impl Cursor {
    pub fn new() -> Self {
        Cursor { x: 0, y: 0 }
    }

    pub fn move_cursor(&mut self, key: KeyCode) {
        match key {
            KeyCode::Up => self.y = self.y.saturating_sub(1),
            KeyCode::Down => self.y = self.y.saturating_add(1),
            KeyCode::Left => self.x = self.x.saturating_sub(1),
            KeyCode::Right => {
                if self.x != *screen::WIDTH - 1 {
                    self.x += 1;
                }
            }

            KeyCode::PageUp => (0..*screen::WIDTH).for_each(|_| {
                self.move_cursor(KeyCode::Up);
            }),

            KeyCode::PageDown => (0..*screen::WIDTH).for_each(|_| {
                self.move_cursor(KeyCode::Down);
            }),

            KeyCode::End => self.x = *screen::WIDTH - 1,
            KeyCode::Home => self.x = 0,

            _ => todo!(),
        };
    }
}

impl Keyboard {
    pub fn new() -> Self {
        Keyboard {
            cursor: Cursor::new(),
        }
    }

    pub fn listener(&self) -> crossterm::Result<KeyEvent> {
        loop {
            if let Event::Key(event) = event::read()? {
                return Ok(event);
            }
        }
    }

    pub fn event(&mut self, key_event: KeyEvent) -> crossterm::Result<bool> {
        match key_event {
            KeyEvent {
                code: KeyCode::Char('c'),
                modifiers: event::KeyModifiers::CONTROL,
                ..
            } => return Ok(false),

            _ => {
                let key = key_event.code;

                if MOVE_CURSOR_KEYS.contains(&key) {
                    self.cursor.move_cursor(key);
                }
            }
        }

        Ok(true)
    }
}
