use std::marker::PhantomData;

use crate::entity::Registry;
use crate::world::Fetch;
use crate::Entity;

pub struct FetchEntity(PhantomData<Entity>);

impl<'data> TryFrom<&'data Registry> for FetchEntity {
    type Error = ();

    fn try_from(_: &'data Registry) -> Result<Self, Self::Error> {
        Ok(Self(PhantomData))
    }
}

impl<'data> Fetch<'data> for FetchEntity {
    type Item = Entity;

    fn fetch(&self, entity: Entity) -> Result<Self::Item, ()> {
        Ok(entity)
    }
}
