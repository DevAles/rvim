pub mod control_devices;
pub mod mode;
pub mod screen;

use crossterm::terminal;

use control_devices::Keyboard;
use screen::Screen;

struct App {
    screen: Screen,
    keyboard: Keyboard,
}

impl App {
    fn new() -> Self {
        App {
            screen: Screen::new(),
            keyboard: Keyboard::new(),
        }
    }

    fn run(&mut self) -> crossterm::Result<bool> {
        self.screen.refresh(&self.keyboard.cursor)?;

        let key = self.keyboard.listener()?;
        self.keyboard.event(key)
    }
}

pub fn run() -> crossterm::Result<()> {
    let _raw_mode_on = mode::RawMode;
    terminal::enable_raw_mode()?;

    let mut app = App::new();
    while app.run()? {}

    Ok(())
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

    let mut keyboard = Keyboard::new();
    keyboard.event(key).unwrap();
}
