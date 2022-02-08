use crate::entity::Entity;
use crate::world::World;

mod tuple;

pub trait Fetch<'data>: TryFrom<&'data World, Error = ()> {
    type Item: Sync + 'data;

    fn fetch(&self, entity: Entity) -> Result<Self::Item, ()>;
}
