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
pub struct FetchResourceWrite<R>
where
    R: Resource,
{
    _ph: PhantomData<R>,
}

#[cfg(feature = "resource")]
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
