use std::ops::Not;

use components::{Mass, Position, Velocity};
use toucan_ecs::world::World;

mod components;

#[test]
fn remove_one() {
    let mut world = World::new();

    let set = {
        let position = Position { x: 1.0, y: 3.0 };
        let velocity = Velocity { dx: 5.0, dy: -10.0 };
        let mass = Mass(10.0);
        (position, velocity, mass)
    };
    let entity = world.create_with(set);
    assert!(world.contains(entity));
    assert!(world.attached::<Position>(entity));
    assert!(world.attached::<Velocity>(entity));
    assert!(world.attached::<Mass>(entity));

    world.remove::<Position>(entity);
    assert!(world.attached::<Position>(entity).not());

    world.remove::<Velocity>(entity);
    assert!(world.attached::<Velocity>(entity).not());

    println!("Mass: {:?}", *world.get::<Mass>(entity).unwrap())
}

#[test]
fn remove() {
    let mut world = World::new();

    let set = {
        let position = Position { x: 1.0, y: 3.0 };
        let velocity = Velocity { dx: 5.0, dy: -10.0 };
        let mass = Mass(15.0);
        (position, velocity, mass)
    };
    let entity = world.create_with(set);

    world.remove::<(Position, Velocity)>(entity);
    assert!(world.attached::<Position>(entity).not());
    assert!(world.attached::<Velocity>(entity).not());

    println!("Mass: {:?}", *world.get::<Mass>(entity).unwrap())
}
