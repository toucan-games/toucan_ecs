use crate::{Entity, Registry};

mod tuple;

pub trait Fetch<'data>: TryFrom<&'data Registry, Error = ()> {
    type Item: Sync + 'data;

    fn fetch(&self, entity: Entity) -> Result<Self::Item, ()>;
}
