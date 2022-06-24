use crate::component::{Component, Storage, StorageImpl};
use crate::world::{Fetch, FetchError, WorldData};
use crate::Entity;

#[repr(transparent)]
pub struct FetchRead<'data, C>
where
    C: Component,
{
    storage: &'data StorageImpl<C>,
}

impl<'data, C> Fetch<'data> for FetchRead<'data, C>
where
    C: Component,
{
    type Item = &'data C;

    // noinspection DuplicatedCode
    fn new(world: WorldData<'data>) -> Result<Self, FetchError> {
        let storage = world.components().get_storage().ok_or(FetchError)?;
        Ok(Self { storage })
    }

    fn fetch(&self, entity: Entity) -> Result<Self::Item, FetchError> {
        self.storage.get(entity).ok_or(FetchError)
    }
}
