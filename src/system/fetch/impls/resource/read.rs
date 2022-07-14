use std::marker::PhantomData;

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

    unsafe fn fetch(world: *mut World) -> FetchResult<Self::Item> {
        let world = &*world;
        let resource = world.get_resource().ok_or(FetchError)?;
        let resource = marker::Resource::new(resource);
        Ok(resource)
    }
}
