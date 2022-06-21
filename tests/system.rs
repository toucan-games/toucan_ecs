use components::{Mass, Position, Velocity};
use toucan_ecs::system::Schedule;

mod components;
#[cfg(feature = "resource")]
mod resources;
mod utils;

fn component_system(position: &mut Position, velocity: &Velocity, mass: &Mass) {
    position.x += 10.0;
    println!(
        "position {:?}, velocity {:?}, mass {:?}",
        position, velocity, mass
    );
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
        .system(component_system)
        .system(local_system)
        .system(|time: Resource<&Time>| println!("Elapsed seconds are {}", time.elapsed_secs()))
        .build();
    schedule.run(&mut world);
}
