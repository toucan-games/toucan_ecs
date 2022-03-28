use atomic_refcell::AtomicRefMut;

use crate::component::{Component, DefaultStorage};
use crate::world::{Fetch, FetchError};
use crate::{Entity, World};

pub struct FetchWrite<'data, C>
where
    C: Component,
{
    storage: &'data DefaultStorage<C>,
}

impl<'data, C> TryFrom<&'data World> for FetchWrite<'data, C>
where
    C: Component,
{
    type Error = FetchError;

    // noinspection DuplicatedCode
    fn try_from(world: &'data World) -> Result<Self, Self::Error> {
        let storage = world.registry().get_storage::<C>().ok_or(FetchError)?;
        Ok(Self { storage })
    }
}

impl<'data, C> Fetch<'data> for FetchWrite<'data, C>
where
    C: Component,
{
    type Item = AtomicRefMut<'data, C>;

    fn fetch(&self, entity: Entity) -> Result<Self::Item, FetchError> {
        self.storage.get_im_mut(entity).ok_or(FetchError)
    }
}
