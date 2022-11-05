use crossterm::terminal;

use crate::screen::Screen;

/// If a RawMode variable goes out of scope, disables terminal raw mode.
pub struct RawMode;

impl Drop for RawMode {
    fn drop(&mut self) {
        terminal::disable_raw_mode().expect("Error turning off raw mode");
        Screen::clear().expect("Clear Screen Error");
    }
}
