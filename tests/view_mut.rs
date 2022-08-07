use components::{Mass, Position, Velocity};
use toucan_ecs::marker::Not;
use toucan_ecs::prelude::*;

mod components;
#[cfg(feature = "resource")]
mod resources;
mod utils;

#[test]
fn view_one_mut() {
    let mut world = utils::prepare_for_view();

    let view_one_mut = world.view_one_mut::<Position>();
    assert_eq!(view_one_mut.len(), 10);

    for (_, component) in view_one_mut {
        component.x -= 10.0;
        println!("component: {:?}", component)
    }
}

#[test]
fn view_mut() {
    let mut world = utils::prepare_for_view();

    type Query<'data> = (
        Entity,
        &'data mut Position,
        &'data Velocity,
        &'data mut Mass,
    );

    for (entity, position, velocity, mass) in world.view_mut::<Query>() {
        position.x += 10.0;
        mass.0 += 1.0;
        println!(
            "entity: {:?}, position: {:?}, velocity: {:?}, mass: {:?}",
            entity, position, velocity, mass,
        )
    }
}

#[test]
fn complex_view_mut() {
    let mut world = utils::prepare_for_complex_view();

    type Query<'data> = (
        Entity,
        &'data mut Position,
        Option<&'data mut Velocity>,
        Not<Mass>,
    );

    for (entity, position, mut velocity, _) in world.view_mut::<Query>() {
        position.y -= 10.0;
        if let Some(velocity) = velocity.as_deref_mut() {
            velocity.dx += 10.0;
        }
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
fn complex_resource_view_mut() {
    use rand::{thread_rng, Rng};
    use resources::SimpleResource;

    let mut world = utils::prepare_for_complex_view();
    world.create_resources(SimpleResource::default());

    type Query<'data> = (
        Entity,
        &'data mut Position,
        Not<Velocity>,
        Option<&'data mut Mass>,
    );

    let (mut components, mut resources) = world.split_mut();
    let resource = resources.get_mut::<SimpleResource>().unwrap();
    for (entity, position, _, mut mass) in components.view_mut::<Query>() {
        position.x -= 10.0;
        if let Some(mass) = mass.as_deref_mut() {
            mass.0 += 1.0;
        }
        resource.set_inner(thread_rng().gen());
        println!(
            "entity: {:?}, position: {:?}, mass: {:?}, inner: {}",
            entity,
            position,
            mass.as_deref(),
            resource.inner(),
        )
    }
}
