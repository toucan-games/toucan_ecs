use crate::component::{Component, Storage, StorageImpl};
use crate::world::{Fetch, FetchError, WorldData};
use crate::Entity;

#[repr(transparent)]
pub struct FetchOptionRead<'data, C>
where
    C: Component,
{
    storage: Option<&'data StorageImpl<C>>,
}

impl<'data, C> Fetch<'data> for FetchOptionRead<'data, C>
where
    C: Component,
{
    type Item = Option<&'data C>;

    // noinspection DuplicatedCode
    fn new(world: WorldData<'data>) -> Result<Self, FetchError> {
        let storage = world.components().get_storage();
        Ok(Self { storage })
    }

    fn fetch(&self, entity: Entity) -> Result<Self::Item, FetchError> {
        let item = self.storage.and_then(|storage| storage.get(entity));
        Ok(item)
    }
}
