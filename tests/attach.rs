use std::ops::Not;

use components::{Mass, Position, Velocity};
use toucan_ecs::Registry;

mod components;

#[test]
fn attach() {
    let mut registry = Registry::new();
    let entity = registry.create();

    assert!(registry.attached::<Position>(entity).not());
    registry.attach(entity, Position { x: 0.0, y: 0.0 });
    assert!(registry.attached::<Position>(entity));

    registry.attach(entity, Velocity { dx: 1.0, dy: 2.0 });
    assert!(registry.attached::<Velocity>(entity));

    assert!(registry.attached::<Mass>(entity).not());
}

#[test]
fn attach_set() {
    let mut registry = Registry::new();

    let entity = registry.create();
    assert!(registry.attached::<Position>(entity).not());
    assert!(registry.attached::<Velocity>(entity).not());
    assert!(registry.attached::<Mass>(entity).not());

    let set = {
        let position = Position { x: 1.0, y: 3.0 };
        let velocity = Velocity { dx: 5.0, dy: -10.0 };
        let mass = Mass(10.0);
        (position, velocity, mass)
    };
    registry.attach_set(entity, set);
    assert!(registry.attached::<Position>(entity));
    assert!(registry.attached::<Velocity>(entity));
    assert!(registry.attached::<Mass>(entity));
}
