use std::fmt::Debug;

use components::{Mass, Position, Velocity};
#[cfg(feature = "resource")]
use resources::SimpleResource;
use toucan_ecs::marker;
use toucan_ecs::prelude::*;
use toucan_ecs::world::query::Query;

mod components;
#[cfg(feature = "resource")]
mod resources;
mod utils;

#[cfg(feature = "resource")]
fn foreach_component_system(
    entity: Entity,
    position: &mut Position,
    velocity: &Velocity,
    mass: Option<&Mass>,
    mut resource: marker::ResourceMut<SimpleResource>,
) {
    position.x += 10.0;
    let inner = {
        let inner = resource.inner();
        resource.set_inner(inner + 1);
        resource.inner()
    };
    println!(
        "entity: {:?}, position: {:?}, velocity: {:?}, mass: {:?}, inner: {}",
        entity,
        position,
        velocity,
        mass.as_deref(),
        inner,
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

    struct MySystem;

    impl<'data> System<'data, ()> for MySystem {
        fn run(&mut self, _: ()) {
            println!("You can create your own systems")
        }
    }

    let mut schedule = Schedule::builder()
        .system(|| println!("Hello, World"))
        .system(|| println!("Result of sum is {}", 2 + 2))
        .system(MySystem)
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
fn for_each_system() {
    use std::io::Read;
    use toucan_ecs::resource::Resource;

    #[derive(Resource)]
    struct File(std::fs::File);

    let mut world = utils::prepare_for_view();
    world.create_resources(SimpleResource::default());

    let mut schedule = Schedule::builder()
        .system(|res: marker::Resource<SimpleResource>| println!("Inner is {}", res.inner()))
        .foreach_system(|| println!("Will be repeated for each entity"))
        .system(|file: Option<marker::ResourceMut<File>>| {
            println!("Is some file: {}", file.is_some());
            if let Some(mut file) = file {
                let mut contents = String::new();
                file.0
                    .read_to_string(&mut contents)
                    .expect("not valid UTF-8");
                println!("file contents: {}", contents);
            }
        })
        .foreach_system(foreach_component_system)
        .build();
    schedule.run(&mut world);
}
