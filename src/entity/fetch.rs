use std::marker::PhantomData;

use crate::world::{Fetch, FetchError, FetchMut, WorldData, WorldDataMut};
use crate::Entity;

pub struct FetchEntity(PhantomData<Entity>);

impl<'data> TryFrom<WorldData<'data>> for FetchEntity {
    type Error = FetchError;

    fn try_from(_: WorldData<'data>) -> Result<Self, Self::Error> {
        Ok(Self(PhantomData))
    }
}

impl<'data> Fetch<'data> for FetchEntity {
    type Item = Entity;

    fn fetch(&self, entity: Entity) -> Result<Self::Item, FetchError> {
        Ok(entity)
    }
}

impl<'data> TryFrom<WorldDataMut<'data>> for FetchEntity {
    type Error = FetchError;

    fn try_from(_: WorldDataMut<'data>) -> Result<Self, Self::Error> {
        Ok(Self(PhantomData))
    }
}

impl<'data> FetchMut<'data> for FetchEntity {
    type Item = Entity;

    unsafe fn fetch_mut(&'data mut self, entity: Entity) -> Result<Self::Item, FetchError> {
        Ok(entity)
    }
}
