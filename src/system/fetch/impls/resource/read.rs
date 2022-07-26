use std::marker::PhantomData;

use atomicell::Ref;

use crate::error::{FetchError, FetchResult};
use crate::resource::{marker, Resource};
use crate::system::fetch::Fetch;
use crate::world::World;

#[repr(transparent)]
pub struct FetchResourceRead<R>
where
    R: Resource,
{
    _ph: PhantomData<R>,
}

impl<'data, R> Fetch<'data> for FetchResourceRead<R>
where
    R: Resource,
{
    type Item = marker::Resource<'data, R>;

    fn fetch(world: &'data World) -> FetchResult<Self::Item> {
        let resource = world.resources().get_guarded().ok_or(FetchError)?;
        let resource = Ref::leak(resource);
        let resource = marker::Resource::new(resource);
        Ok(resource)
    }
}
