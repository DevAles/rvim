pub mod mode;
pub mod tools;
pub mod screen;

use crossterm::terminal;

use tools::keyboard;
use screen::Screen;

pub fn run() -> crossterm::Result<()> {
    let mut screen = Screen::new()?;
    screen.refresh()?;
    screen.create_rows();


    let _raw_mode_on = mode::RawMode;
    terminal::enable_raw_mode()?;

    loop {
        let key = keyboard::listener()?;

        println!("{:?}\r", key);
        screen.refresh()?;

        if !keyboard::event(key)? {
            Screen::clear()?;
            return Ok(());
        }
    }
}

#[cfg(test)]
#[test]
fn keyboard_event() {
    use crossterm::event::{
        KeyCode::Char, KeyEvent, KeyEventKind::Press, KeyEventState, KeyModifiers,
    };

    let key = KeyEvent {
        code: Char('a'),
        modifiers: KeyModifiers::NONE,
        kind: Press,
        state: KeyEventState::NONE,
    };
    keyboard::event(key).unwrap();
}
