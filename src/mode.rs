use crossterm::terminal;

/// If a RawMode variable goes out of scope, disables terminal raw mode.
pub struct RawMode;

impl Drop for RawMode {
    fn drop(&mut self) {
        terminal::disable_raw_mode().expect("Error turning off raw mode");
    }
}