use toucan_ecs::system::Schedule;
use toucan_ecs::World;

#[test]
fn test() {
    let mut world = World::new();

    let mut schedule = Schedule::builder()
        .system(|| println!("Hello, World"))
        .system(|| println!("Result of sum is {}", 2 + 2))
        .build();
    schedule.run(&mut world);
}
