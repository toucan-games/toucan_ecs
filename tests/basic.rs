use components::{Position, Velocity};
use toucan_ecs::Registry;

mod components;

#[test]
fn initialization() {
    let _ = Registry::new();
}

#[test]
fn components() {
    let mut registry = Registry::new();

    let entity = registry.create();
    assert!(registry.attached(entity));

    registry.attach(entity, Position { x: 0.0, y: 0.0 });
    registry.attach(entity, Velocity { dx: 1.0, dy: 2.0 });
}
