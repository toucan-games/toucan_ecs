use crate::component::{Component, DefaultStorage};
use crate::entity::Entity;
use crate::world::{Fetch, World};

pub struct FetchNot<'data, C>
where
    C: Component,
{
    storage: Option<&'data DefaultStorage<C>>,
}

impl<'data, C> TryFrom<&'data World> for FetchNot<'data, C>
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

impl<'data, C> Fetch<'data> for FetchNot<'data, C>
where
    C: Component,
{
    type Item = ();

    fn fetch(&self, entity: Entity) -> Result<Self::Item, ()> {
        match self.storage {
            None => Ok(()),
            Some(storage) => {
                let component = storage.get(entity);
                match component {
                    None => Ok(()),
                    Some(_) => Err(()),
                }
            }
        }
    }
}
