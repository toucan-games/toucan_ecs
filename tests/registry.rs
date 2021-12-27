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

#[test]
fn integration() {
    let mut registry = Registry::new();
    registry.register::<Position>();

    for i in 0..=10 {
        let position = Position {
            x: i as f32,
            y: i as f32,
        };
        let entity = registry.create_with(position);
        assert!(registry.contains(entity));
    }

    for (entity, position) in registry.view::<Position>() {
        println!("entity: {:?}, position: {:?}", entity, position)
    }
}

#[derive(Copy, Clone, Debug)]
struct Position {
    x: f32,
    y: f32,
}

#[derive(Copy, Clone, Debug)]
struct Velocity {
    dx: f32,
    dy: f32,
}
