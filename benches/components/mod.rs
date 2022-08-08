use toucan_ecs::prelude::*;

#[derive(Copy, Clone, Component, Debug, PartialEq)]
pub struct Position {
    pub x: f32,
    pub y: f32,
}

#[derive(Copy, Clone, Component, Debug, PartialEq)]
pub struct Velocity {
    pub dx: f32,
    pub dy: f32,
}

#[derive(Copy, Clone, Component, Debug, PartialEq)]
pub struct Mass(pub f32);
