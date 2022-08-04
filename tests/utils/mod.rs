use toucan_ecs::prelude::*;

use crate::components::{Mass, Position, Velocity};

pub fn prepare_for_view() -> World {
    let mut world = World::new();

    for i in 1..=10 {
        let f = i as f32;
        let position = Position { x: f, y: -f };
        let velocity = Velocity {
            dx: f / 10.0,
            dy: -f / 10.0,
        };
        let mass = Mass(f);
        let _entity = world.create_with((position, velocity, mass));
    }

    world
}

#[allow(dead_code)]
pub fn prepare_for_complex_view() -> World {
    let mut world = World::new();

    for i in 1..=10 {
        let f = i as f32;
        let position = Position { x: f, y: -f };
        let velocity = Velocity {
            dx: f / 10.0,
            dy: -f / 10.0,
        };
        let mass = Mass(f);
        let entity = world.create();
        world.attach(entity, position);
        if i % 2 == 0 {
            world.attach(entity, velocity);
        } else {
            world.attach(entity, mass);
        }
    }

    world
}
