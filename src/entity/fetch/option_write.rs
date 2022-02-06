use crate::component::{DefaultStorage, RefMut};
use crate::world::Fetch;
use crate::{Component, Entity, World};

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
    type Error = ();

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
    type Item = Option<RefMut<'data, C>>;

    fn fetch(&self, entity: Entity) -> Result<Self::Item, ()> {
        let item = self.storage.and_then(|storage| storage.get_mut(entity));
        Ok(item)
    }
}
