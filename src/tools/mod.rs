use crossterm::event::{self, KeyEvent, KeyCode, Event};

pub struct EventTrigger;
pub struct KeyboardListener;

impl KeyboardListener {
    pub fn on() -> crossterm::Result<KeyEvent> {
        loop {
            if let Event::Key(event) = event::read()? {
                return Ok(event);
            }
        }
    }
}

impl EventTrigger {
    pub fn on(event: KeyEvent) -> crossterm::Result<bool> {
        match event {
            KeyEvent {
                code: KeyCode::Char('c'),
                modifiers: event::KeyModifiers::CONTROL,
                ..
            } => return Ok(false),

            _ => { /* todo */ }
        }

        Ok(true)
    }
}
