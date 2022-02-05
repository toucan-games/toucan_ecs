use crate::component::storage::DefaultStorage;
use crate::{Component, Entity, RefMut, Registry};

use super::Fetch;

pub struct FetchWrite<'data, C>
where
    C: Component,
{
    storage: &'data DefaultStorage<C>,
}

impl<'data, C> TryFrom<&'data Registry> for FetchWrite<'data, C>
where
    C: Component,
{
    type Error = ();

    // noinspection DuplicatedCode
    fn try_from(registry: &'data Registry) -> Result<Self, Self::Error> {
        let storage = registry.get_storage::<C>().ok_or(())?;
        Ok(Self { storage })
    }
}

impl<'data, C> Fetch<'data> for FetchWrite<'data, C>
where
    C: Component,
{
    type Item = RefMut<'data, C>;

    fn fetch(&self, entity: Entity) -> Result<Self::Item, ()> {
        self.storage.get_mut(entity).ok_or(())
    }
}
