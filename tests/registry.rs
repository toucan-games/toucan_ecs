use toucan_ecs::Registry;

#[test]
fn register() {
    let mut registry = Registry::new();

    let entity = registry.create();
    assert!(registry.contains(entity));

    registry.register::<Position>();
    registry.add(entity, Position { x: 0.0, y: 0.0 });

    registry.register::<Velocity>();
    registry.add(entity, Velocity { dx: 1.0, dy: 2.0 });
}

#[test]
#[should_panic]
fn no_register() {
    let mut registry = Registry::new();

    let entity = registry.create();
    assert!(registry.contains(entity));

    registry.add(entity, Position { x: 0.0, y: 0.0 });
}

#[derive(Copy, Clone)]
struct Position {
    x: f32,
    y: f32,
}

#[derive(Copy, Clone)]
struct Velocity {
    dx: f32,
    dy: f32,
}
