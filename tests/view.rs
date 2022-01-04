use components::{Mass, Position, Velocity};
use toucan_ecs::Registry;

mod components;

#[test]
fn view() {
    let mut registry = Registry::new();

    for i in 0..=10 {
        let f = i as f32;
        let position = Position { x: f, y: f };
        let velocity = Velocity {
            dx: f / 10.0,
            dy: -f / 10.0,
        };
        let mass = Mass(f);
        let entity = registry.create();
        if let Some(mut entry) = registry.entry(entity) {
            entry.attach_set((position, velocity, mass));
        }
        assert!(registry.attached(entity));
    }

    for (entity, component) in registry.view::<Position>() {
        println!("entity: {:?}, component: {:?}", entity, component)
    }
}

#[test]
fn view_mut() {}
