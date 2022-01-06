use components::{Mass, Position, Velocity};
use toucan_ecs::Registry;

mod components;

#[test]
fn create() {
    let mut registry = Registry::new();

    let entity = registry.create();
    assert!(registry.contains(entity));
}

#[test]
fn create_with() {
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
}
