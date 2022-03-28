use std::marker::PhantomData;

use atomic_refcell::AtomicRefMut;

use crate::resource::Resource;
use crate::world::{Fetch, FetchError};
use crate::{Entity, World};

pub struct FetchWrite<'data, R>
where
    R: Resource,
{
    world: &'data World,
    _ph: PhantomData<AtomicRefMut<'data, R>>,
}

impl<'data, R> TryFrom<&'data World> for FetchWrite<'data, R>
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

impl<'data, R> Fetch<'data> for FetchWrite<'data, R>
where
    R: Resource,
{
    type Item = AtomicRefMut<'data, R>;

    fn fetch(&self, _: Entity) -> Result<Self::Item, FetchError> {
        let resource = self.world.resources().get_im_mut().ok_or(FetchError)?;
        Ok(resource)
    }
}
