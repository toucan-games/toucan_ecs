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

    for mut component in world.view_one_mut::<Position>() {
        component.x -= 10.0;
        println!("component: {:?}", *component)
    }
}

#[test]
fn view_mut() {
    let mut world = utils::prepare_for_view();

    for (entity, mut position, velocity, mut mass) in
        world.view_mut::<(Entity, &mut Position, &Velocity, &mut Mass)>()
    {
        position.x -= 10.0;
        mass.0 += 1.0;
        println!(
            "entity: {:?}, position: {:?}, velocity: {:?}, mass: {:?}",
            entity, *position, *velocity, *mass,
        )
    }
}

#[test]
fn complex_view_mut() {
    let mut world = utils::prepare_for_complex_view();

    for (entity, mut position, _, mut mass) in
        world.view_mut::<(Entity, &mut Position, Not<Velocity>, Option<&mut Mass>)>()
    {
        position.x -= 10.0;
        if let Some(ref mut mass) = mass {
            mass.0 += 1.0;
        }
        println!(
            "entity: {:?}, position: {:?}, mass: {:?}",
            entity,
            *position,
            mass.as_deref(),
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

    for (entity, mut position, _, mut mass, mut time) in world.view_mut::<(
        Entity,
        &mut Position,
        Not<Velocity>,
        Option<&mut Mass>,
        Resource<&mut Time>,
    )>() {
        position.x -= 10.0;
        if let Some(ref mut mass) = mass {
            mass.0 += 1.0;
        }
        time.reset();
        println!(
            "entity: {:?}, position: {:?}, mass: {:?}, time: {}",
            entity,
            *position,
            mass.as_deref(),
            time.elapsed_secs(),
        )
    }
}
