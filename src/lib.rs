pub mod mode;
pub mod tools;

use crossterm::terminal;
use tools::{EventTrigger, KeyboardListener};

pub fn run() -> crossterm::Result<()> {
    let _raw_mode_on = mode::RawMode;
    terminal::enable_raw_mode()?;

    loop {
        let key = KeyboardListener::on()?;
        println!("{:?}\r", key);

        if !EventTrigger::on(key)? {
            return Ok(())
        }
    }
}

#[cfg(test)]
#[test]
fn event_trigger() {
    use crossterm::event::{ KeyEvent, KeyCode::Char, KeyModifiers, KeyEventKind::Press, KeyEventState };

    let key = KeyEvent { code: Char('a'), modifiers: KeyModifiers::NONE, kind: Press, state: KeyEventState::NONE };
    EventTrigger::on(key).unwrap();
}