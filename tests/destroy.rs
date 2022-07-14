use std::ops::Not;

use components::{Mass, Position, Velocity};
use toucan_ecs::world::World;

mod components;

#[test]
fn destroy() {
    let mut world = World::new();

    let entity = world.create();
    assert!(world.contains(entity));

    world.destroy(entity);
    assert!(world.contains(entity).not());
}

#[test]
fn destroy_with_data() {
    let mut world = World::new();

    let set = {
        let position = Position { x: 1.0, y: 3.0 };
        let velocity = Velocity { dx: 5.0, dy: -10.0 };
        let mass = Mass(10.0);
        (position, velocity, mass)
    };
    let entity = world.create_with(set);
    assert!(world.contains(entity));

    world.destroy(entity);
    assert!(world.contains(entity).not());
    assert!(world.attached_one::<Position>(entity).not());
    assert!(world.attached_one::<Velocity>(entity).not());
    assert!(world.attached_one::<Mass>(entity).not());
}
