use crate::component::{Component, Storage, StorageImpl};
use crate::world::{FetchError, FetchMut};
use crate::{Entity, World};

#[repr(transparent)]
pub struct FetchWriteMut<'data, C>
where
    C: Component,
{
    storage: &'data mut StorageImpl<C>,
}

impl<'data, C> TryFrom<&'data mut World> for FetchWriteMut<'data, C>
where
    C: Component,
{
    type Error = FetchError;

    fn try_from(world: &'data mut World) -> Result<Self, Self::Error> {
        let storage = world.components_mut().get_storage_mut().ok_or(FetchError)?;
        Ok(Self { storage })
    }
}

impl<'data, C> FetchMut<'data> for FetchWriteMut<'data, C>
where
    C: Component,
{
    type Item = &'data mut C;

    fn fetch_mut(&'data mut self, entity: Entity) -> Result<Self::Item, FetchError> {
        self.storage.get_mut(entity).ok_or(FetchError)
    }
}
