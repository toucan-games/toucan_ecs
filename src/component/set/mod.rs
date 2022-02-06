use crate::entity::Registry;
use crate::Entity;

mod tuple;

pub trait ComponentSet {
    fn attach(self, registry: &mut Registry, entity: Entity);

    fn remove(registry: &mut Registry, entity: Entity);

    fn attached(registry: &Registry, entity: Entity) -> bool;
}
