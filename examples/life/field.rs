pub struct Field {
    width: u16,
}

impl Field {
    pub fn new(width: u16) -> Self {
        Self { width }
    }

    pub fn width(&self) -> u16 {
        self.width
    }
}

#[derive(Copy, Clone, Debug)]
pub struct Point {
    pub x: u16,
    pub y: u16,
}

#[derive(Copy, Clone, Debug)]
pub struct Alive {
    pub alive: bool,
}

#[derive(Copy, Clone, Debug)]
pub struct WatchAfter;
