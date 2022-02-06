use components::{Mass, Position, Velocity};
use toucan_ecs::World;

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
