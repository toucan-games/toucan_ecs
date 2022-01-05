use std::ops::Not;

use components::{Mass, Position, Velocity};
use toucan_ecs::Registry;

mod components;

#[test]
fn destroy() {
    let mut registry = Registry::new();

    let entity = registry.create();
    assert!(registry.contains(entity));

    registry.destroy(entity);
    assert!(registry.contains(entity).not());
}

#[test]
fn destroy_with_data() {
    let mut registry = Registry::new();

    let set = {
        let position = Position { x: 1.0, y: 3.0 };
        let velocity = Velocity { dx: 5.0, dy: -10.0 };
        let mass = Mass(10.0);
        (position, velocity, mass)
    };
    let entity = registry.create_with(set);
    assert!(registry.contains(entity));

    registry.destroy(entity);
    assert!(registry.contains(entity).not());
    assert!(registry.attached::<Position>(entity).not());
    assert!(registry.attached::<Velocity>(entity).not());
    assert!(registry.attached::<Mass>(entity).not());
}
