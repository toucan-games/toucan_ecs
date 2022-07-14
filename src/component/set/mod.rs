use crate::component::{Component, Registry};
use crate::entity::Entity;

mod tuple;

pub trait ComponentSet {
    fn attach(self, registry: &mut Registry, entity: Entity);

    fn remove(registry: &mut Registry, entity: Entity);

    fn attached(registry: &Registry, entity: Entity) -> bool;
}

impl<C> ComponentSet for C
where
    C: Component,
{
    fn attach(self, registry: &mut Registry, entity: Entity) {
        registry.attach_one(entity, self)
    }

    fn remove(registry: &mut Registry, entity: Entity) {
        registry.remove_one::<Self>(entity)
    }

    fn attached(registry: &Registry, entity: Entity) -> bool {
        registry.attached_one::<Self>(entity)
    }
}
