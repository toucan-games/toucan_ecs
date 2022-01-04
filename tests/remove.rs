use std::ops::Not;

use components::{Mass, Position, Velocity};
use toucan_ecs::Registry;

mod components;

#[test]
fn remove() {
    let mut registry = Registry::new();

    let set = {
        let position = Position { x: 1.0, y: 3.0 };
        let velocity = Velocity { dx: 5.0, dy: -10.0 };
        let mass = Mass(10.0);
        (position, velocity, mass)
    };
    let entity = registry.create_with(set);
    assert!(registry.contains(entity));
    assert!(registry.attached::<Position>(entity));
    assert!(registry.attached::<Velocity>(entity));
    assert!(registry.attached::<Mass>(entity));

    registry.remove::<Position>(entity);
    assert!(registry.attached::<Position>(entity).not());

    registry.remove::<Velocity>(entity);
    assert!(registry.attached::<Velocity>(entity).not());

    println!("Mass: {:?}", registry.get::<Mass>(entity).unwrap())
}

#[test]
fn remove_set() {
    let mut registry = Registry::new();

    let set = {
        let position = Position { x: 1.0, y: 3.0 };
        let velocity = Velocity { dx: 5.0, dy: -10.0 };
        let mass = Mass(15.0);
        (position, velocity, mass)
    };
    let entity = registry.create_with(set);

    registry.remove_set::<(Position, Velocity)>(entity);
    assert!(registry.attached::<Position>(entity).not());
    assert!(registry.attached::<Velocity>(entity).not());

    println!("Mass: {:?}", registry.get::<Mass>(entity).unwrap())
}
