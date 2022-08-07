use components::{Mass, Position, Velocity};
use toucan_ecs::marker::Not;
use toucan_ecs::prelude::*;

mod components;
#[cfg(feature = "resource")]
mod resources;
mod utils;

#[test]
fn view_one() {
    let world = utils::prepare_for_view();

    let view_one = world.view_one::<Position>();
    assert_eq!(view_one.len(), 10);

    for component in view_one {
        println!("component: {:?}", component)
    }
}

#[test]
fn view() {
    let world = utils::prepare_for_view();

    type Query<'data> = (Entity, &'data Position, &'data Velocity, &'data Mass);

    for (entity, position, velocity, mass) in world.view::<Query>() {
        println!(
            "entity: {:?}, position: {:?}, velocity: {:?}, mass: {:?}",
            entity, position, velocity, mass,
        )
    }
}

#[test]
fn complex_view() {
    let world = utils::prepare_for_complex_view();

    type Query<'data> = (Entity, &'data Position, Option<&'data Velocity>, Not<Mass>);

    for (entity, position, velocity, _) in world.view::<Query>() {
        println!(
            "entity: {:?}, position: {:?}, velocity: {:?}",
            entity,
            position,
            velocity.as_deref(),
        )
    }
}

#[test]
#[cfg(feature = "resource")]
fn complex_resource_view() {
    use resources::SimpleResource;

    let mut world = utils::prepare_for_complex_view();
    world.create_resources(SimpleResource::default());

    type Query<'data> = (Entity, &'data Position, Option<&'data Velocity>, Not<Mass>);

    let (components, resources) = world.split();
    for (entity, position, velocity, _) in components.view::<Query>() {
        let resource = resources.get::<SimpleResource>().unwrap();
        println!(
            "entity: {:?}, position: {:?}, velocity: {:?}, inner: {}",
            entity,
            position,
            velocity.as_deref(),
            resource.inner(),
        )
    }
}
