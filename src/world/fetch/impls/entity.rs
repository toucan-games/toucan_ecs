use crate::error::FetchResult;
use crate::world::{Fetch, FetchMut, WorldData, WorldDataMut};
use crate::Entity;

pub struct FetchEntity;

impl<'data> Fetch<'data> for FetchEntity {
    type Item = Entity;

    fn new(_: WorldData<'data>) -> FetchResult<Self> {
        Ok(Self)
    }

    fn fetch(&self, entity: Entity) -> FetchResult<Self::Item> {
        Ok(entity)
    }
}

impl<'data> FetchMut<'data> for FetchEntity {
    type Item = Entity;

    unsafe fn new(_: WorldDataMut<'data>) -> FetchResult<Self> {
        Ok(Self)
    }

    fn fetch_mut(&'data mut self, entity: Entity) -> FetchResult<Self::Item> {
        Ok(entity)
    }
}
