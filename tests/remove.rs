use std::ops::Not;

use components::{Mass, Position, Velocity};
use toucan_ecs::Registry;

mod components;

#[test]
fn remove_one() {
    let mut registry = Registry::new();

    let set = {
        let position = Position { x: 1.0, y: 3.0 };
        let velocity = Velocity { dx: 5.0, dy: -10.0 };
        let mass = Mass(10.0);
        (position, velocity, mass)
    };
    let entity = registry.create_with(set);
    assert!(registry.contains(entity));
    assert!(registry.attached_one::<Position>(entity));
    assert!(registry.attached_one::<Velocity>(entity));
    assert!(registry.attached_one::<Mass>(entity));

    registry.remove_one::<Position>(entity);
    assert!(registry.attached_one::<Position>(entity).not());

    registry.remove_one::<Velocity>(entity);
    assert!(registry.attached_one::<Velocity>(entity).not());

    println!("Mass: {:?}", registry.get::<Mass>(entity).unwrap())
}

#[test]
fn remove() {
    let mut registry = Registry::new();

    let set = {
        let position = Position { x: 1.0, y: 3.0 };
        let velocity = Velocity { dx: 5.0, dy: -10.0 };
        let mass = Mass(15.0);
        (position, velocity, mass)
    };
    let entity = registry.create_with(set);

    registry.remove::<(Position, Velocity)>(entity);
    assert!(registry.attached_one::<Position>(entity).not());
    assert!(registry.attached_one::<Velocity>(entity).not());

    println!("Mass: {:?}", registry.get::<Mass>(entity).unwrap())
}
