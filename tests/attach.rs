use std::ops::Not;

use components::{Mass, Position, Velocity};
use toucan_ecs::world::World;

mod components;

#[test]
fn attach_one() {
    let mut world = World::new();
    let entity = world.create();

    assert!(world.attached::<Position>(entity).not());
    world.attach(entity, Position { x: 0.0, y: 0.0 });
    assert!(world.attached::<Position>(entity));

    world.attach(entity, Velocity { dx: 1.0, dy: 2.0 });
    assert!(world.attached::<Velocity>(entity));

    assert!(world.attached::<Mass>(entity).not());
}

#[test]
fn attach() {
    let mut world = World::new();

    let entity = world.create();
    assert!(world.attached::<Position>(entity).not());
    assert!(world.attached::<Velocity>(entity).not());
    assert!(world.attached::<Mass>(entity).not());

    let set = {
        let position = Position { x: 1.0, y: 3.0 };
        let velocity = Velocity { dx: 5.0, dy: -10.0 };
        let mass = Mass(10.0);
        (position, velocity, mass)
    };
    world.attach(entity, set);
    assert!(world.attached::<Position>(entity));
    assert!(world.attached::<Velocity>(entity));
    assert!(world.attached::<Mass>(entity));
}
