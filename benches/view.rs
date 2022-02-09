use criterion::{black_box, criterion_group, criterion_main, BatchSize, Criterion};

use components::{Mass, Position, Velocity};
use toucan_ecs::World;

mod components;

fn setup() -> World {
    let mut world = World::new();
    world.register::<Position>();
    world.register::<Velocity>();
    world.register::<Mass>();

    for i in 0..1_000_u16 {
        let f = f32::from(i);
        let position = Position { x: f, y: -f };
        let velocity = Velocity {
            dx: f / 10.0,
            dy: -f / 10.0,
        };
        let mass = Mass(f);

        let mut entry = world.create_entry();
        entry.attach_one(position);
        if i % 2 == 0 {
            entry.attach_one(velocity);
        } else {
            entry.attach_one(mass);
        }
    }

    world
}

fn view(criterion: &mut Criterion) {
    fn routine(world: World) {
        let view = world.view::<(&Position, Option<&Velocity>, &Mass)>();
        view.for_each(|item| {
            black_box(item);
        });
    }

    criterion.bench_function("view world", |bencher| {
        bencher.iter_batched(setup, routine, BatchSize::SmallInput)
    });
}

fn view_mut(criterion: &mut Criterion) {
    fn routine(mut world: World) {
        let view = world.view_mut::<(&mut Position, Option<&mut Velocity>, &mut Mass)>();
        view.for_each(|item| {
            let (mut position, velocity, mut mass) = item;
            position.x += black_box(1.0);
            mass.0 -= black_box(0.1);
            if let Some(mut velocity) = velocity {
                velocity.dy -= black_box(1.0);
            }
        });
    }

    criterion.bench_function("view mut world", |bencher| {
        bencher.iter_batched(setup, routine, BatchSize::SmallInput)
    });
}

criterion_group!(view_group, view, view_mut);
criterion_main!(view_group);
