use std::fmt::Debug;

use components::{Mass, Position, Velocity};
use toucan_ecs::component::{Component, ViewOne, ViewOneMut};
use toucan_ecs::system::Schedule;
use toucan_ecs::world::{View, ViewMut};
use toucan_ecs::Entity;

mod components;
#[cfg(feature = "resource")]
mod resources;
mod utils;

fn for_each_component_system(position: &mut Position, velocity: &Velocity, mass: Option<&Mass>) {
    position.x += 10.0;
    println!(
        "position {:?}, velocity {:?}, mass {:?}",
        position,
        velocity,
        mass.as_deref()
    );
}

fn view_one_system<C>(view_one: ViewOne<C>)
where
    C: Component + Debug,
{
    for (entity, component) in view_one {
        println!("entity: {:?}, component: {:?}", entity, component)
    }
}

fn view_one_mut_system(view_one_mut: ViewOneMut<Velocity>) {
    for (entity, velocity) in view_one_mut {
        velocity.dx = 0.0;
        velocity.dy = 0.0;
        println!("entity: {:?}, velocity: {:?}", entity, velocity)
    }
}

fn view_system<'data>(view: View<'data, (Entity, &'data Position)>) {
    for (entity, position) in view {
        println!("entity: {:?}, position: {:?}", entity, position)
    }
}

fn view_mut_system<'data>(view_mut: ViewMut<'data, (Entity, &'data mut Position)>) {
    for (entity, position) in view_mut {
        position.x = 0.0;
        position.y = 0.0;
        println!("entity: {:?}, position: {:?}", entity, position)
    }
}

#[test]
#[cfg(feature = "resource")]
fn test() {
    use resources::Time;
    use toucan_ecs::resource::marker::Resource;

    let mut world = utils::prepare_for_view();
    world.create_resource(Time::new());

    let mut local_var = 0;
    let local_system = || {
        local_var += 1;
        println!("Some var is {}", local_var)
    };

    let mut schedule = Schedule::builder()
        .system(|| println!("Hello, World"))
        .system(|| println!("Result of sum is {}", 2 + 2))
        .system(for_each_component_system)
        .system(local_system)
        .system(|time: Resource<Time>| println!("Elapsed seconds are {}", time.elapsed_secs()))
        .system(view_one_system::<Position>)
        .system(view_one_mut_system)
        .system(view_system)
        .system(view_mut_system)
        .build();
    schedule.run(&mut world);
}
