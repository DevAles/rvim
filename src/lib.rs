use crossterm::event;
use crossterm::event::Event;
use crossterm::event::KeyCode;
use crossterm::event::KeyEvent;
use crossterm::terminal;

struct RawModeOn;

impl Drop for RawModeOn {
    fn drop(&mut self) {
        terminal::disable_raw_mode().expect("Error turning off raw mode");
    }
}

pub fn run() -> crossterm::Result<()> {
    // If a RawModeOn variable goes out of scope, turn off raw mode to avoid keeping raw mode on if the program panics or something else
    let _raw_mode_on = RawModeOn;
    terminal::enable_raw_mode()?;

    loop {
        if let Event::Key(event) = event::read()? {
            match event {
                KeyEvent {
                    code: KeyCode::Char('q'),
                    modifiers: event::KeyModifiers::NONE,
                    ..
                } => break,

                _ => { /* todo */ }
            }

            println!("{:?}\r", event); // Print event
        }
    }
    Ok(())
}