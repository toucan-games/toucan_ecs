use components::{Mass, Position, Velocity};
use toucan_ecs::Entity;

mod components;
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

    for (entity, position, velocity, mass) in
        registry.view::<(Entity, &Position, &Velocity, &Mass)>()
    {
        println!(
            "entity: {:?}, position: {:?}, velocity: {:?}, mass: {:?}",
            entity, *position, *velocity, *mass,
        )
    }
}

#[test]
fn option_view() {
    let registry = utils::prepare_for_optional_view();

    for (entity, position, velocity, mass) in
        registry.view::<(Entity, &Position, Option<&Velocity>, Option<&Mass>)>()
    {
        println!(
            "entity: {:?}, position: {:?}, velocity: {:?}, mass: {:?}",
            entity,
            *position,
            velocity.as_deref(),
            mass.as_deref(),
        )
    }
}
