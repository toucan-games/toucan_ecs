use crate::component::{Component, Storage, StorageImpl};
use crate::world::{FetchError, FetchMut, WorldDataMut};
use crate::Entity;

#[repr(transparent)]
pub struct FetchReadMut<'data, C>
where
    C: Component,
{
    storage: &'data StorageImpl<C>,
}

impl<'data, C> FetchMut<'data> for FetchReadMut<'data, C>
where
    C: Component,
{
    type Item = &'data C;

    unsafe fn new(world: WorldDataMut<'data>) -> Result<Self, FetchError> {
        // SAFETY: must be checked by the caller.
        let storage = world.components().get_storage().ok_or(FetchError)?;
        Ok(Self { storage })
    }

    fn fetch_mut(&mut self, entity: Entity) -> Result<Self::Item, FetchError> {
        self.storage.get(entity).ok_or(FetchError)
    }
}
