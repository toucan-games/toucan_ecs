use crate::entity::{Entity, Registry};

mod tuple;

pub trait ComponentSet {
    fn attach(self, registry: &mut Registry, entity: Entity);

    fn remove(registry: &mut Registry, entity: Entity);

    fn attached(registry: &Registry, entity: Entity) -> bool;
}