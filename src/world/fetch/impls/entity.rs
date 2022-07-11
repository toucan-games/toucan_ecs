use crate::entity::Entity;
use crate::error::FetchResult;
use crate::fetch::FetchEntity;
use crate::world::{Fetch, FetchMut, WorldData, WorldDataMut};

impl<'data> Fetch<'data> for FetchEntity {
    type Item = Entity;

    fn new(_: WorldData<'data>) -> FetchResult<Self> {
        Ok(Self)
    }

    fn entities(&self) -> Option<Box<dyn ExactSizeIterator<Item = Entity> + Send + Sync + 'data>> {
        None
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

    fn entities(&self) -> Option<Box<dyn ExactSizeIterator<Item = Entity> + Send + Sync + 'data>> {
        None
    }

    fn fetch_mut(&'data mut self, entity: Entity) -> FetchResult<Self::Item> {
        Ok(entity)
    }
}
