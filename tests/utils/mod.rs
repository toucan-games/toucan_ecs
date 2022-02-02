use toucan_ecs::Registry;

use crate::components::{Mass, Position, Velocity};

pub fn prepare_for_view() -> Registry {
    let mut registry = Registry::new();

    for i in 0..=10 {
        let f = i as f32;
        let position = Position { x: f, y: -f };
        let velocity = Velocity {
            dx: f / 10.0,
            dy: -f / 10.0,
        };
        let mass = Mass(f);
        let _entity = registry.create_with((position, velocity, mass));
    }

    registry
}
