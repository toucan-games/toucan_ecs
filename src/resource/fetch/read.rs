use std::marker::PhantomData;

use crate::entity::Entity;
use crate::resource::{Ref, Resource};
use crate::world::{Fetch, World};

pub struct FetchRead<'data, R>
where
    R: Resource,
{
    world: &'data World,
    _ph: PhantomData<Ref<'data, R>>,
}

impl<'data, R> TryFrom<&'data World> for FetchRead<'data, R>
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

impl<'data, R> Fetch<'data> for FetchRead<'data, R>
where
    R: Resource,
{
    type Item = Ref<'data, R>;

    fn fetch(&self, _: Entity) -> Result<Self::Item, ()> {
        let resource = self.world.resources().get().ok_or(())?;
        Ok(resource)
    }
}
