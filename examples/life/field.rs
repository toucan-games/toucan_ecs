use toucan_ecs::component::Component;

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

#[derive(Copy, Clone, Component, Debug)]
pub struct Point {
    pub x: u16,
    pub y: u16,
}

#[derive(Copy, Clone, Component, Debug)]
pub struct Alive;

#[derive(Copy, Clone, Component, Debug)]
pub struct WatchAfter;
