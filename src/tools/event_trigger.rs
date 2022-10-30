use crossterm::event::{self, KeyCode, KeyEvent};

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