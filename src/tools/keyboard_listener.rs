use crossterm::event::{self, Event, KeyEvent};

pub fn on() -> crossterm::Result<KeyEvent> {
    loop {
        if let Event::Key(event) = event::read()? {
            return Ok(event);
        }
    }
}