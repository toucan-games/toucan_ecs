use criterion::{criterion_group, criterion_main, BatchSize, Criterion};

use components::{Mass, Position, Velocity};
use toucan_ecs::world::World;

mod components;

fn create_world(criterion: &mut Criterion) {
    fn routine() {
        let _world = World::new();
    }

    criterion.bench_function("create world", |bencher| bencher.iter(routine));
}

fn register_world_components(criterion: &mut Criterion) {
    fn routine(mut world: World) {
        world.register::<Position>();
        world.register::<Velocity>();
        world.register::<Mass>();
    }

    criterion.bench_function("register world components", |bencher| {
        bencher.iter_batched(World::new, routine, BatchSize::SmallInput)
    });
}

fn fill_world(criterion: &mut Criterion) {
    fn setup() -> World {
        let mut world = World::new();
        world.register::<Position>();
        world.register::<Velocity>();
        world.register::<Mass>();
        world
    }

    fn routine(mut world: World) {
        let into_iter = (0..1_000_u16).map(|i| {
            let f = f32::from(i);
            let velocity = Velocity {
                dx: -f / 10.0,
                dy: f / 10.0,
            };
            let position = Position { x: -f, y: f };
            let mass = Mass(f);
            (velocity, position, mass)
        });
        world.extend_with(into_iter);
    }

    criterion.bench_function("fill world", |bencher| {
        bencher.iter_batched(setup, routine, BatchSize::SmallInput)
    });
}

criterion_group!(
    world_group,
    create_world,
    register_world_components,
    fill_world,
);
criterion_main!(world_group);
