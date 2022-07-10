use std::fmt::Debug;

use components::{Mass, Position, Velocity};
use toucan_ecs::component::Component;
use toucan_ecs::system::Schedule;
use toucan_ecs::world::query::Query;
use toucan_ecs::world::view::{View, ViewMut, ViewOne, ViewOneMut};
use toucan_ecs::Entity;

mod components;
#[cfg(feature = "resource")]
mod resources;
mod utils;

fn for_each_component_system(
    entity: Entity,
    position: &mut Position,
    velocity: &Velocity,
    mass: Option<&Mass>,
) {
    position.x += 10.0;
    println!(
        "entity: {:?}, position: {:?}, velocity: {:?}, mass: {:?}",
        entity,
        position,
        velocity,
        mass.as_deref(),
    );
}

fn view_one_system<C>(view_one: ViewOne<C>)
where
    C: Component + Debug,
{
    assert_eq!(view_one.len(), 10);
    for (entity, component) in view_one {
        println!("entity: {:?}, component: {:?}", entity, component)
    }
}

fn view_one_mut_system(view_one_mut: ViewOneMut<Velocity>) {
    assert_eq!(view_one_mut.len(), 10);
    for (entity, velocity) in view_one_mut {
        velocity.dx = 0.0;
        velocity.dy = 0.0;
        println!("entity: {:?}, velocity: {:?}", entity, velocity)
    }
}

fn view_system<'data, Q>(view: View<'data, Q>)
where
    Q: Query<'data> + Debug,
{
    for item in view {
        println!("item: {:?}", item)
    }
}

fn view_mut_system<'data>(view_mut: ViewMut<'data, (Entity, &'data mut Position)>) {
    for (entity, position) in view_mut {
        position.x = 0.0;
        position.y = 0.0;
        println!("entity: {:?}, position: {:?}", entity, position)
    }
}

fn complex_view_mut_system<'data>(
    _view_mut: ViewMut<'data, (Entity, &'data mut Position, Option<&'data Mass>)>,
    _velocities: ViewMut<'data, &'data mut Velocity>,
    masses: ViewOne<'data, Mass>,
) {
    assert_eq!(masses.len(), 10);
    // do nothing here for now
}

#[test]
fn system() {
    let mut world = utils::prepare_for_view();

    let mut local_var = 0;
    let local_system = || {
        local_var += 1;
        println!("Some var is {}", local_var)
    };

    let mut schedule = Schedule::builder()
        .system(|| println!("Hello, World"))
        .system(|| println!("Result of sum is {}", 2 + 2))
        .system(local_system)
        .system(view_one_system::<Position>)
        .system(view_one_mut_system)
        .system(view_system::<(Entity, &Position, Option<&Velocity>)>)
        .system(view_mut_system)
        .system(complex_view_mut_system)
        .build();
    schedule.run(&mut world);
}

#[test]
#[cfg(feature = "resource")]
#[cfg(not(miri))]
fn for_each_system() {
    use resources::SimpleResource;
    use toucan_ecs::resource::marker::Resource;

    let mut world = utils::prepare_for_view();
    world.create_resource(SimpleResource::default());

    let mut schedule = Schedule::builder()
        .system::<_, (_,)>(|res: Resource<SimpleResource>| println!("Inner is {}", res.inner()))
        .system(for_each_component_system)
        .build();
    schedule.run(&mut world);
}
