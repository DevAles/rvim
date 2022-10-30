use std::io::stdout;

use crossterm::{ execute, terminal, terminal::ClearType, cursor::MoveTo };

pub fn clear() -> crossterm::Result<()> {
    execute!(stdout(), terminal::Clear(ClearType::All))?;
    execute!(stdout(), MoveTo(0, 0))
}