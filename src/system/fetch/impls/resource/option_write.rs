use std::marker::PhantomData;

use atomicell::RefMut;

use crate::error::FetchResult;
use crate::resource::{marker, Resource};
use crate::system::fetch::Fetch;
use crate::world::World;

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

    fn fetch(world: &'data World) -> FetchResult<Self::Item> {
        let resource = world.resources().get_mut_guarded().map(RefMut::leak);
        let resource = resource.map(marker::ResourceMut::new);
        Ok(resource)
    }
}
