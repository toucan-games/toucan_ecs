use std::marker::PhantomData;

use crate::resource::Resource;
use crate::world::{Fetch, FetchError};
use crate::{Entity, World};

pub struct FetchRead<'data, R>
where
    R: Resource,
{
    world: &'data World,
    _ph: PhantomData<&'data R>,
}

impl<'data, R> TryFrom<&'data World> for FetchRead<'data, R>
where
    R: Resource,
{
    type Error = FetchError;

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
    type Item = &'data R;

    fn fetch(&self, _: Entity) -> Result<Self::Item, FetchError> {
        let resource = self.world.resources().get().ok_or(FetchError)?;
        Ok(resource)
    }
}
