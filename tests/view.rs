use components::{Mass, Position, Velocity};
use toucan_ecs::component::marker::Not;
use toucan_ecs::Entity;

mod components;
#[cfg(feature = "resource")]
mod resources;
mod utils;

#[test]
fn view_one() {
    let world = utils::prepare_for_view();

    for component in world.view_one::<Position>() {
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
    use toucan_ecs::resource::marker::Resource;

    let mut world = utils::prepare_for_complex_view();
    world.create_resource(SimpleResource::default());

    type Query<'data> = (
        Entity,
        &'data Position,
        Option<&'data Velocity>,
        Not<Mass>,
        Resource<'data, SimpleResource>,
    );

    for (entity, position, velocity, _, time) in world.view::<Query>() {
        println!(
            "entity: {:?}, position: {:?}, velocity: {:?}, inner: {}",
            entity,
            position,
            velocity.as_deref(),
            time.inner(),
        )
    }
}
