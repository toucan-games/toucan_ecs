use components::{Mass, Position, Velocity};
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

    assert_eq!(registry.get::<Position>(entity), None);
    registry.attach(entity, Position { x: 0.0, y: 0.0 });
    assert_ne!(registry.get::<Position>(entity), None);

    registry.attach(entity, Velocity { dx: 1.0, dy: 2.0 });
    assert_eq!(
        registry.get::<Velocity>(entity),
        Some(&Velocity { dx: 1.0, dy: 2.0 }),
    );

    assert_eq!(registry.get::<Mass>(entity), None);
}
