use crate::resource::{Registry, Resource};

mod tuple;

pub trait ResourceSet {
    fn create(self, registry: &mut Registry);

    fn destroy(registry: &mut Registry);

    fn contains(registry: &Registry) -> bool;
}

impl<R> ResourceSet for R
where
    R: Resource,
{
    fn create(self, registry: &mut Registry) {
        registry.create_one(self)
    }

    fn destroy(registry: &mut Registry) {
        registry.destroy_one::<Self>()
    }

    fn contains(registry: &Registry) -> bool {
        registry.contains_one::<Self>()
    }
}
