use crate::component::{Component, Storage, StorageImpl};
use crate::world::{FetchError, FetchMut, WorldDataMut};
use crate::Entity;

#[repr(transparent)]
pub struct FetchOptionWriteMut<'data, C>
where
    C: Component,
{
    storage: Option<&'data mut StorageImpl<C>>,
}

impl<'data, C> FetchMut<'data> for FetchOptionWriteMut<'data, C>
where
    C: Component,
{
    type Item = Option<&'data mut C>;

    unsafe fn new(world: WorldDataMut<'data>) -> Result<Self, FetchError> {
        // SAFETY: must be checked by the caller.
        let storage = world.components_mut().get_storage_mut();
        Ok(Self { storage })
    }

    fn fetch_mut(&'data mut self, entity: Entity) -> Result<Self::Item, FetchError> {
        let storage = self.storage.as_mut();
        let item = storage.and_then(|storage| storage.get_mut(entity));
        Ok(item)
    }
}
