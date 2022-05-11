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
fn view_one_mut() {
    let mut world = utils::prepare_for_view();

    for component in world.view_one_mut::<Position>() {
        component.x += 10.0;
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
fn complex_view() {
    let world = utils::prepare_for_complex_view();

    type Query<'data> = (
        Entity,
        &'data Position,
        Option<&'data Velocity>,
        Not<'data, Mass>,
    );

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
fn complex_view_mut() {
    let mut world = utils::prepare_for_complex_view();

    type Query<'data> = (
        Entity,
        &'data mut Position,
        Option<&'data mut Velocity>,
        Not<'data, Mass>,
    );

    for (entity, position, mut velocity, _) in world.view_mut::<Query>() {
        position.y -= 10.0;
        match velocity.as_deref_mut() {
            Some(velocity) => velocity.dx += 10.0,
            None => {}
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
fn complex_resource_view() {
    use resources::Time;
    use toucan_ecs::resource::marker::Resource;

    let mut world = utils::prepare_for_complex_view();
    world.create_resource(Time::new());

    type Query<'data> = (
        Entity,
        &'data Position,
        Option<&'data Velocity>,
        Not<'data, Mass>,
        Resource<&'data Time>,
    );

    for (entity, position, velocity, _, time) in world.view::<Query>() {
        println!(
            "entity: {:?}, position: {:?}, velocity: {:?}, time: {}",
            entity,
            position,
            velocity.as_deref(),
            time.elapsed_secs(),
        )
    }
}

#[test]
#[cfg(feature = "resource")]
fn complex_resource_view_mut() {
    use resources::Time;
    use toucan_ecs::resource::marker::Resource;

    let mut world = utils::prepare_for_complex_view();
    world.create_resource(Time::new());

    type Query<'data> = (
        Entity,
        &'data mut Position,
        Option<&'data mut Velocity>,
        Not<'data, Mass>,
        Resource<&'data mut Time>,
    );

    for (entity, position, mut velocity, _, time) in world.view_mut::<Query>() {
        position.x += 10.0;
        match velocity.as_deref_mut() {
            Some(velocity) => velocity.dx += 10.0,
            None => {}
        }
        time.reset();
        println!(
            "entity: {:?}, position: {:?}, velocity: {:?}, time: {}",
            entity,
            position,
            velocity.as_deref(),
            time.elapsed_secs(),
        )
    }
}
