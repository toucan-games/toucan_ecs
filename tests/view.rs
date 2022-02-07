use components::{Mass, Position, Velocity};
use resources::Time;
use toucan_ecs::{Entity, Not, Res};

mod components;
mod resources;
mod utils;

#[test]
fn view_one() {
    let registry = utils::prepare_for_view();

    for component in registry.view_one::<Position>() {
        println!("component: {:?}", *component)
    }
}

#[test]
fn view() {
    let registry = utils::prepare_for_view();

    for (entity, position, velocity, mass, time) in
        registry.view::<(Entity, &Position, &Velocity, &Mass, Res<&Time>)>()
    {
        println!(
            "entity: {:?}, position: {:?}, velocity: {:?}, mass: {:?}, time: {}",
            entity,
            *position,
            *velocity,
            *mass,
            time.elapsed_secs(),
        )
    }
}

#[test]
fn complex_view() {
    let registry = utils::prepare_for_complex_view();

    for (entity, position, velocity, _, time) in
        registry.view::<(Entity, &Position, Option<&Velocity>, Not<Mass>, Res<&Time>)>()
    {
        println!(
            "entity: {:?}, position: {:?}, velocity: {:?}, time: {}",
            entity,
            *position,
            velocity.as_deref(),
            time.elapsed_secs(),
        )
    }
}
