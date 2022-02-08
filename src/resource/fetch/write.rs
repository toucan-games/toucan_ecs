use std::marker::PhantomData;

use crate::entity::Entity;
use crate::resource::{RefMut, Resource};
use crate::world::{Fetch, World};

pub struct FetchWrite<'data, R>
where
    R: Resource,
{
    world: &'data World,
    _ph: PhantomData<RefMut<'data, R>>,
}

impl<'data, R> TryFrom<&'data World> for FetchWrite<'data, R>
where
    R: Resource,
{
    type Error = ();

    fn try_from(world: &'data World) -> Result<Self, Self::Error> {
        Ok(Self {
            world,
            _ph: PhantomData,
        })
    }
}

impl<'data, R> Fetch<'data> for FetchWrite<'data, R>
where
    R: Resource,
{
    type Item = RefMut<'data, R>;

    fn fetch(&self, _: Entity) -> Result<Self::Item, ()> {
        let resource = self.world.resources().get_mut().ok_or(())?;
        Ok(resource)
    }
}
