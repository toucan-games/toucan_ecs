use std::marker::PhantomData;

use crate::world::{Fetch, FetchError, FetchMut, WorldData, WorldDataMut};
use crate::Entity;

pub struct FetchEntity(PhantomData<Entity>);

impl<'data> Fetch<'data> for FetchEntity {
    type Item = Entity;

    fn new(_: WorldData<'data>) -> Result<Self, FetchError> {
        Ok(Self(PhantomData))
    }

    fn fetch(&self, entity: Entity) -> Result<Self::Item, FetchError> {
        Ok(entity)
    }
}

impl<'data> FetchMut<'data> for FetchEntity {
    type Item = Entity;

    unsafe fn new(_: WorldDataMut<'data>) -> Result<Self, FetchError> {
        Ok(Self(PhantomData))
    }

    fn fetch_mut(&'data mut self, entity: Entity) -> Result<Self::Item, FetchError> {
        Ok(entity)
    }
}
