use std::marker::PhantomData;

use crate::error::FetchResult;
use crate::resource::{marker, Resource};
use crate::system::fetch::Fetch;
use crate::World;

#[repr(transparent)]
pub struct FetchResourceOptionRead<R>
where
    R: Resource,
{
    _ph: PhantomData<R>,
}

impl<'data, R> Fetch<'data> for FetchResourceOptionRead<R>
where
    R: Resource,
{
    type Item = Option<marker::Resource<'data, R>>;

    unsafe fn fetch(world: *mut World) -> FetchResult<Self::Item> {
        let world = &*world;
        let resource = world.get_resource().map(marker::Resource::new);
        Ok(resource)
    }
}
