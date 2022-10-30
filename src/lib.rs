pub mod mode;
pub mod tools;

use crossterm::terminal;
use tools::{ event_trigger, keyboard_listener, screen };

pub fn run() -> crossterm::Result<()> {
    screen::clear()?;

    let _raw_mode_on = mode::RawMode;
    terminal::enable_raw_mode()?;

    loop {
        let key = keyboard_listener::on()?;
        println!("{:?}\r", key);

        if !event_trigger::on(key)? {
            return Ok(())
        }
    }
}

#[cfg(test)]
#[test]
fn event_trigger() {
    use crossterm::event::{ KeyEvent, KeyCode::Char, KeyModifiers, KeyEventKind::Press, KeyEventState };

    let key = KeyEvent { code: Char('a'), modifiers: KeyModifiers::NONE, kind: Press, state: KeyEventState::NONE };
    event_trigger::on(key).unwrap();
}