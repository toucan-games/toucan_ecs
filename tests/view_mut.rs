use components::{Mass, Position, Velocity};
use resources::Time;
use toucan_ecs::{Entity, ResourceWrite};

mod components;
mod resources;
mod utils;

#[test]
fn view_one_mut() {
    let mut registry = utils::prepare_for_view();

    for mut component in registry.view_one_mut::<Position>() {
        component.x -= 10.0;
        println!("component: {:?}", *component)
    }
}

#[test]
fn view_mut() {
    let mut registry = utils::prepare_for_view();

    for (entity, mut position, velocity, mut mass, mut time) in registry.view_mut::<(
        Entity,
        &mut Position,
        &Velocity,
        &mut Mass,
        ResourceWrite<Time>,
    )>() {
        position.x -= 10.0;
        mass.0 += 1.0;
        time.reset();
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
fn option_view_mut() {
    let mut registry = utils::prepare_for_optional_view();

    for (entity, mut position, velocity, mut mass) in
        registry.view_mut::<(Entity, &mut Position, Option<&Velocity>, Option<&mut Mass>)>()
    {
        position.x -= 10.0;
        if let Some(ref mut mass) = mass {
            mass.0 += 1.0;
        }
        println!(
            "entity: {:?}, position: {:?}, velocity: {:?}, mass: {:?}",
            entity,
            *position,
            velocity.as_deref(),
            mass.as_deref(),
        )
    }
}
