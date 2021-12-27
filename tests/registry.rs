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
    registry.register::<Velocity>();
    registry.register::<Mass>();

    for i in 0..=10 {
        let i = i as f32;
        let position = Position { x: i, y: i };
        let velocity = Velocity {
            dx: i / 10.0,
            dy: -i / 10.0,
        };
        let mass = Mass(i);
        let entity = registry.create_with((position, velocity, mass));
        assert!(registry.contains(entity));
    }

    for (entity, component) in registry.view::<Position>() {
        println!("entity: {:?}, component: {:?}", entity, component)
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

#[derive(Copy, Clone, Debug)]
struct Mass(f32);
