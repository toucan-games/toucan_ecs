use crate::entity::Entity;
use crate::error::FetchResult;
use crate::fetch::FetchEntity;
use crate::system::foreach::fetch::Fetch;
use crate::world::WorldDataMut;

impl<'data> Fetch<'data> for FetchEntity {
    type Item = Entity;

    unsafe fn new(_: WorldDataMut<'data>) -> FetchResult<Self> {
        Ok(Self)
    }

    fn entities(&self) -> Option<Box<dyn ExactSizeIterator<Item = Entity> + Send + Sync + 'data>> {
        None
    }

    fn fetch(&'data mut self, entity: Entity) -> FetchResult<Self::Item> {
        Ok(entity)
    }
}
