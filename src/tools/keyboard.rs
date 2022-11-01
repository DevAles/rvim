use crossterm::event::{self, Event, KeyEvent, KeyCode};

pub fn listener() -> crossterm::Result<KeyEvent> {
    loop {
        if let Event::Key(event) = event::read()? {
            return Ok(event);
        }
    }
}

pub fn event(key_event: KeyEvent) -> crossterm::Result<bool> {
    match key_event {
        KeyEvent {
            code: KeyCode::Char('c'),
            modifiers: event::KeyModifiers::CONTROL,
            ..
        } => return Ok(false),

        _ => { /* todo */ }
    }

    Ok(true)
}