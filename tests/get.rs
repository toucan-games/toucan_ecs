use components::Velocity;
use toucan_ecs::world::World;

mod components;

#[test]
fn get() {
    let mut world = World::new();
    let entity = world.create_with_one(Velocity {
        dx: 10.0,
        dy: -10.0,
    });

    let data = world.get::<Velocity>(entity).unwrap();
    assert_eq!(
        *data,
        Velocity {
            dx: 10.0,
            dy: -10.0,
        }
    );
}

#[test]
fn get_mut() {
    let mut world = World::new();
    let entity = world.create_with_one(Velocity {
        dx: 10.0,
        dy: -10.0,
    });

    let data = world.get_mut::<Velocity>(entity).unwrap();
    assert_eq!(
        *data,
        Velocity {
            dx: 10.0,
            dy: -10.0,
        }
    );
}

#[test]
fn get_mut_and_mutate() {
    let mut world = World::new();
    let entity = world.create_with_one(Velocity {
        dx: 10.0,
        dy: -10.0,
    });

    let mut data = world.get_mut::<Velocity>(entity).unwrap();
    data.dx = 0.0;
    data.dy = 0.0;
    assert_eq!(*data, Velocity { dx: 0.0, dy: 0.0 });

    let data = *data;
    assert_eq!(data, *world.get::<Velocity>(entity).unwrap());
}
