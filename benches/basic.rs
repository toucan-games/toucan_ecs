use criterion::{criterion_group, criterion_main, Criterion};

use toucan_ecs::World;

mod components;

fn create_registry(criterion: &mut Criterion) {
    fn create_registry() {
        let _world = World::new();
    }

    criterion.bench_function("create world", |bencher| bencher.iter(create_registry));
}

criterion_group!(basic_benches, create_registry);
criterion_main!(basic_benches);
