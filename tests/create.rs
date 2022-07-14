use components::{Mass, Position, Velocity};
use toucan_ecs::world::World;

mod components;

#[test]
fn create() {
    let mut world = World::new();

    let entity = world.create();
    assert!(world.contains(entity));
}

#[test]
fn create_with() {
    let mut world = World::new();

    let set = {
        let position = Position { x: 1.0, y: 3.0 };
        let velocity = Velocity { dx: 5.0, dy: -10.0 };
        let mass = Mass(10.0);
        (position, velocity, mass)
    };
    let entity = world.create_with(set);

    assert!(world.contains(entity));
    assert!(world.attached_one::<Position>(entity));
    assert!(world.attached_one::<Velocity>(entity));
    assert!(world.attached_one::<Mass>(entity));
}

#[test]
fn create_with_builder() {
    let mut world = World::new();

    let builder = world.entity().with(Position { x: 0.0, y: 0.0 });
    let mass = Mass(100.0);
    let mut builder = builder.with(mass);
    if mass.0 > 10.0 {
        let value = mass.0 * 10.0;
        builder = builder.with(Velocity {
            dx: value,
            dy: value,
        })
    }

    let entity = builder.build();
    assert!(world.contains(entity));
    assert_eq!(world.get(entity), Some(&Position { x: 0.0, y: 0.0 }));
    assert_eq!(world.get(entity), Some(&Mass(100.0)));
    assert_eq!(
        world.get(entity),
        Some(&Velocity {
            dx: 1000.0,
            dy: 1000.0,
        })
    );
}
