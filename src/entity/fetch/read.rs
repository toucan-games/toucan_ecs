use crate::component::{DefaultStorage, Ref};
use crate::world::Fetch;
use crate::{Component, Entity, World};

pub struct FetchRead<'data, C>
where
    C: Component,
{
    storage: &'data DefaultStorage<C>,
}

impl<'data, C> TryFrom<&'data World> for FetchRead<'data, C>
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

impl<'data, C> Fetch<'data> for FetchRead<'data, C>
where
    C: Component,
{
    type Item = Ref<'data, C>;

    fn fetch(&self, entity: Entity) -> Result<Self::Item, ()> {
        self.storage.get(entity).ok_or(())
    }
}
