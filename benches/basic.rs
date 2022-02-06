use criterion::{criterion_group, criterion_main, Criterion};

use toucan_ecs::Registry;

mod components;

fn create_registry(criterion: &mut Criterion) {
    fn create_registry() {
        let _registry = Registry::new();
    }

    criterion.bench_function("create registry", |bencher| bencher.iter(create_registry));
}

criterion_group!(basic_benches, create_registry);
criterion_main!(basic_benches);
