use std::marker::PhantomData;

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

    unsafe fn fetch(world: *mut World) -> FetchResult<Self::Item> {
        let world = &mut *world;
        let resource = world.get_resource_mut().ok_or(FetchError)?;
        let resource = marker::ResourceMut::new(resource);
        Ok(resource)
    }
}
