use components::{Mass, Position, Velocity};
use toucan_ecs::component::marker::Not;
use toucan_ecs::Entity;

mod components;
#[cfg(feature = "resource")]
mod resources;
mod utils;

#[test]
fn view_one_mut() {
    let mut world = utils::prepare_for_view();

    for (_, component) in world.view_one_mut::<Position>() {
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
    use resources::Time;
    use toucan_ecs::resource::marker::ResourceMut;

    let mut world = utils::prepare_for_complex_view();
    world.create_resource(Time::new());

    type Query<'data> = (
        Entity,
        &'data mut Position,
        Not<Velocity>,
        Option<&'data mut Mass>,
        ResourceMut<'data, Time>,
    );

    for (entity, position, _, mut mass, mut time) in world.view_mut::<Query>() {
        position.x -= 10.0;
        if let Some(mass) = mass.as_deref_mut() {
            mass.0 += 1.0;
        }
        time.reset();
        println!(
            "entity: {:?}, position: {:?}, mass: {:?}, time: {}",
            entity,
            position,
            mass.as_deref(),
            time.elapsed_secs(),
        )
    }
}
