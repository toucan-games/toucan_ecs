use std::marker::PhantomData;

use crate::error::FetchResult;
use crate::resource::{marker, Resource};
use crate::system::fetch::Fetch;
use crate::World;

#[repr(transparent)]
pub struct FetchResourceOptionWrite<R>
where
    R: Resource,
{
    _ph: PhantomData<R>,
}

impl<'data, R> Fetch<'data> for FetchResourceOptionWrite<R>
where
    R: Resource,
{
    type Item = Option<marker::ResourceMut<'data, R>>;

    unsafe fn fetch(world: *mut World) -> FetchResult<Self::Item> {
        let world = &mut *world;
        let resource = world.get_resource_mut().map(marker::ResourceMut::new);
        Ok(resource)
    }
}
