use components::{Mass, Position};
use toucan_ecs::world::World;

mod components;

#[test]
fn extend() {
    let mut world = World::new();

    let entities = world.extend(10).to_vec();
    assert!(!world.is_empty());
    assert_eq!(entities.len(), 10);

    for entity in entities {
        assert!(world.is_entity_empty(entity));
    }
}

#[test]
fn extend_with_one() {
    let mut world = World::new();

    let entities = world.extend_with_one([Mass(0.0); 10]).to_vec();
    assert!(!world.is_empty());
    assert_eq!(entities.len(), 10);

    for entity in entities {
        assert!(!world.is_entity_empty(entity));
        assert!(world.attached_one::<Mass>(entity));
    }
}

#[test]
fn extend_with() {
    let mut world = World::new();

    let entities = world
        .extend_with(std::iter::repeat((Mass(1.0), Position { x: 0.0, y: 0.0 })).take(10))
        .to_vec();
    assert!(!world.is_empty());
    assert_eq!(entities.len(), 10);

    for entity in entities {
        assert!(!world.is_entity_empty(entity));
        assert!(world.attached::<(Position, Mass)>(entity));
    }
}
