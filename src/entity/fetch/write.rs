use crate::component::{DefaultStorage, RefMut};
use crate::world::Fetch;
use crate::{Component, Entity, World};

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
    type Error = ();

    // noinspection DuplicatedCode
    fn try_from(world: &'data World) -> Result<Self, Self::Error> {
        let storage = world.registry().get_storage::<C>().ok_or(())?;
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
