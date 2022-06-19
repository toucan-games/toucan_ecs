use crate::component::{Component, Storage, StorageImpl};
use crate::world::{FetchError, FetchMut};
use crate::{Entity, World};

#[repr(transparent)]
pub struct FetchOptionWriteMut<'data, C>
where
    C: Component,
{
    storage: Option<&'data mut StorageImpl<C>>,
}

impl<'data, C> TryFrom<&'data mut World> for FetchOptionWriteMut<'data, C>
where
    C: Component,
{
    type Error = FetchError;

    fn try_from(world: &'data mut World) -> Result<Self, Self::Error> {
        let storage = world.components_mut().get_storage_mut();
        Ok(Self { storage })
    }
}

impl<'data, C> FetchMut<'data> for FetchOptionWriteMut<'data, C>
where
    C: Component,
{
    type Item = Option<&'data mut C>;

    fn fetch_mut(&'data mut self, entity: Entity) -> Result<Self::Item, FetchError> {
        let storage = self.storage.as_mut();
        let item = storage.and_then(|storage| storage.get_mut(entity));
        Ok(item)
    }
}
