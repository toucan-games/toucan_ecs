#[cfg(feature = "resource")]
use std::marker::PhantomData;

#[cfg(feature = "resource")]
use crate::error::FetchError;
#[cfg(feature = "resource")]
use crate::error::FetchResult;
#[cfg(feature = "resource")]
use crate::resource::{marker, Resource};
#[cfg(feature = "resource")]
use crate::system::fetch::Fetch;
#[cfg(feature = "resource")]
use crate::World;

#[cfg(feature = "resource")]
pub struct FetchResourceRead<R>
where
    R: Resource,
{
    _ph: PhantomData<R>,
}

#[cfg(feature = "resource")]
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
