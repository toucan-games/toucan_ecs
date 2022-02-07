use toucan_ecs::World;

use crate::components::{Mass, Position, Velocity};
use crate::resources::Time;

pub fn prepare_for_view() -> World {
    let mut world = World::new();

    world.create_resource(Time::new());
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

pub fn prepare_for_complex_view() -> World {
    let mut world = World::new();

    world.create_resource(Time::new());
    for i in 1..=10 {
        let f = i as f32;
        let position = Position { x: f, y: -f };
        let velocity = Velocity {
            dx: f / 10.0,
            dy: -f / 10.0,
        };
        let mass = Mass(f);
        let entity = world.create();
        world.attach_one(entity, position);
        if i % 2 == 0 {
            world.attach_one(entity, velocity);
        } else {
            world.attach_one(entity, mass);
        }
    }

    world
}
