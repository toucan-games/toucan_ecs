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
    assert!(registry.attached(entity));
    assert_ne!(registry.get::<Position>(entity), None);
    assert_ne!(registry.get::<Velocity>(entity), None);
    assert_ne!(registry.get::<Mass>(entity), None);

    registry.remove::<Position>(entity);
    assert_eq!(registry.get::<Position>(entity), None);

    registry.remove::<Velocity>(entity);
    assert_eq!(registry.get::<Velocity>(entity), None);

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
    assert_eq!(registry.get::<Position>(entity), None);
    assert_eq!(registry.get::<Velocity>(entity), None);

    println!("Mass: {:?}", registry.get::<Mass>(entity).unwrap())
}
