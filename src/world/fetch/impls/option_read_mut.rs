use crate::component::{Component, StorageHolder};
use crate::error::FetchResult;
use crate::world::{FetchMut, WorldDataMut};
use crate::Entity;

#[repr(transparent)]
pub struct FetchOptionReadMut<'data, C>
where
    C: Component,
{
    storage: Option<StorageHolder<'data, C>>,
}

impl<'data, C> FetchMut<'data> for FetchOptionReadMut<'data, C>
where
    C: Component,
{
    type Item = Option<&'data C>;

    // noinspection DuplicatedCode
    unsafe fn new(world: WorldDataMut<'data>) -> FetchResult<Self> {
        // SAFETY: must be checked by the caller.
        let storage = world.components().get_storage();
        Ok(Self { storage })
    }

    // noinspection DuplicatedCode
    fn entities(&self) -> Option<Box<dyn ExactSizeIterator<Item = Entity> + Send + Sync + 'data>> {
        let iter = self.storage.as_ref()?.iter();
        let iter = iter.map(|(entity, _)| entity);
        Some(Box::new(iter))
    }

    fn fetch_mut(&'data mut self, entity: Entity) -> FetchResult<Self::Item> {
        let item = self.storage.and_then(|storage| storage.get(entity));
        Ok(item)
    }
}
