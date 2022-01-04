use components::{Mass, Position, Velocity};
use toucan_ecs::Registry;

mod components;

#[test]
fn initialization() {
    let _ = Registry::new();
}

#[test]
fn attach() {
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

#[test]
fn attach_set() {
    let mut registry = Registry::new();

    let entity = registry.create();
    assert_eq!(registry.get::<Position>(entity), None);
    assert_eq!(registry.get::<Velocity>(entity), None);
    assert_eq!(registry.get::<Mass>(entity), None);

    let set = {
        let position = Position { x: 1.0, y: 3.0 };
        let velocity = Velocity { dx: 5.0, dy: -10.0 };
        let mass = Mass(10.0);
        (position, velocity, mass)
    };
    registry.attach_set(entity, set);
    assert_eq!(
        registry.get::<Position>(entity),
        Some(&Position { x: 1.0, y: 3.0 }),
    );
    assert_eq!(
        registry.get::<Velocity>(entity),
        Some(&Velocity { dx: 5.0, dy: -10.0 }),
    );
    assert_eq!(registry.get::<Mass>(entity), Some(&Mass(10.0)));
}
