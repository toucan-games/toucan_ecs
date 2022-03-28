use atomic_refcell::AtomicRefMut;

use crate::component::{Component, DefaultStorage};
use crate::world::{Fetch, FetchError};
use crate::{Entity, World};

pub struct FetchOptionWrite<'data, C>
where
    C: Component,
{
    storage: Option<&'data DefaultStorage<C>>,
}

impl<'data, C> TryFrom<&'data World> for FetchOptionWrite<'data, C>
where
    C: Component,
{
    type Error = FetchError;

    // noinspection DuplicatedCode
    fn try_from(world: &'data World) -> Result<Self, Self::Error> {
        let storage = world.registry().get_storage::<C>();
        Ok(Self { storage })
    }
}

impl<'data, C> Fetch<'data> for FetchOptionWrite<'data, C>
where
    C: Component,
{
    type Item = Option<AtomicRefMut<'data, C>>;

    fn fetch(&self, entity: Entity) -> Result<Self::Item, FetchError> {
        let item = self.storage.and_then(|storage| storage.get_im_mut(entity));
        Ok(item)
    }
}
