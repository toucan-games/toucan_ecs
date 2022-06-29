use crate::component::{Component, StorageHolder};
use crate::error::FetchResult;
use crate::world::{Fetch, WorldData};
use crate::Entity;

#[repr(transparent)]
pub struct FetchOptionRead<'data, C>
where
    C: Component,
{
    storage: Option<StorageHolder<'data, C>>,
}

impl<'data, C> Fetch<'data> for FetchOptionRead<'data, C>
where
    C: Component,
{
    type Item = Option<&'data C>;

    // noinspection DuplicatedCode
    fn new(world: WorldData<'data>) -> FetchResult<Self> {
        let storage = world.components().get_storage();
        Ok(Self { storage })
    }

    // noinspection DuplicatedCode
    fn entities(&self) -> Option<Box<dyn ExactSizeIterator<Item = Entity> + Send + Sync + 'data>> {
        let iter = self.storage.as_ref()?.iter();
        let iter = iter.map(|(entity, _)| entity);
        Some(Box::new(iter))
    }

    fn fetch(&self, entity: Entity) -> FetchResult<Self::Item> {
        let item = self.storage.and_then(|storage| storage.get(entity));
        Ok(item)
    }
}
