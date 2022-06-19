use crate::component::{Component, Storage, StorageImpl};
use crate::world::{FetchError, FetchMut};
use crate::{Entity, World};

#[repr(transparent)]
pub struct FetchReadMut<'data, C>
where
    C: Component,
{
    storage: &'data StorageImpl<C>,
}

impl<'data, C> TryFrom<&'data mut World> for FetchReadMut<'data, C>
where
    C: Component,
{
    type Error = FetchError;

    fn try_from(world: &'data mut World) -> Result<Self, Self::Error> {
        let storage = world.components().get_storage().ok_or(FetchError)?;
        Ok(Self { storage })
    }
}

impl<'data, C> FetchMut<'data> for FetchReadMut<'data, C>
where
    C: Component,
{
    type Item = &'data C;

    fn fetch_mut(&mut self, entity: Entity) -> Result<Self::Item, FetchError> {
        self.storage.get(entity).ok_or(FetchError)
    }
}
