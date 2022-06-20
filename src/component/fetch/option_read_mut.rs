use crate::component::{Component, Storage, StorageImpl};
use crate::world::{FetchError, FetchMut, WorldDataMut};
use crate::Entity;

#[repr(transparent)]
pub struct FetchOptionReadMut<'data, C>
where
    C: Component,
{
    storage: Option<&'data StorageImpl<C>>,
}

impl<'data, C> TryFrom<WorldDataMut<'data>> for FetchOptionReadMut<'data, C>
where
    C: Component,
{
    type Error = FetchError;

    // noinspection DuplicatedCode
    fn try_from(world: WorldDataMut<'data>) -> Result<Self, Self::Error> {
        // SAFETY: must be checked by the caller.
        let storage = unsafe { world.components() }.get_storage();
        Ok(Self { storage })
    }
}

impl<'data, C> FetchMut<'data> for FetchOptionReadMut<'data, C>
where
    C: Component,
{
    type Item = Option<&'data C>;

    unsafe fn fetch_mut(&'data mut self, entity: Entity) -> Result<Self::Item, FetchError> {
        let item = self.storage.and_then(|storage| storage.get(entity));
        Ok(item)
    }
}
