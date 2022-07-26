use std::marker::PhantomData;

use atomicell::RefMut;

use crate::error::{FetchError, FetchResult};
use crate::resource::{marker, Resource};
use crate::system::fetch::Fetch;
use crate::world::World;

#[repr(transparent)]
pub struct FetchResourceWrite<R>
where
    R: Resource,
{
    _ph: PhantomData<R>,
}

impl<'data, R> Fetch<'data> for FetchResourceWrite<R>
where
    R: Resource,
{
    type Item = marker::ResourceMut<'data, R>;

    fn fetch(world: &'data World) -> FetchResult<Self::Item> {
        let resource = world.resources().get_mut_guarded().ok_or(FetchError)?;
        let resource = RefMut::leak(resource);
        let resource = marker::ResourceMut::new(resource);
        Ok(resource)
    }
}
