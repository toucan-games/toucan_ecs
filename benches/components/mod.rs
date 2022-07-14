use toucan_ecs::component::storage::DefaultStorage;
use toucan_ecs::component::Component;

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
