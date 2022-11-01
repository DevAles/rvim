pub struct CursorController {
    pub x: usize,
    pub y: usize
}

impl CursorController {
    pub fn new() -> Self {
        CursorController {
            x: 0,
            y: 0
        }
    }
}