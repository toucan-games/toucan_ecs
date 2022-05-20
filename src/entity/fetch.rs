use std::marker::PhantomData;

use crate::world::{Fetch, FetchError, FetchMut};
use crate::{Entity, World};

pub struct FetchEntity(PhantomData<Entity>);

impl<'data> TryFrom<&'data World> for FetchEntity {
    type Error = FetchError;

    fn try_from(_: &'data World) -> Result<Self, Self::Error> {
        Ok(Self(PhantomData))
    }
}

impl<'data> Fetch<'data> for FetchEntity {
    type Item = Entity;

    fn fetch(&self, entity: Entity) -> Result<Self::Item, FetchError> {
        Ok(entity)
    }
}

impl<'data> FetchMut<'data> for FetchEntity {
    type Item = Entity;
}
