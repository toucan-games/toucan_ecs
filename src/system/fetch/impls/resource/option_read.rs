use std::marker::PhantomData;

use atomicell::Ref;

use crate::error::FetchResult;
use crate::resource::{marker, Resource};
use crate::system::fetch::Fetch;
use crate::world::World;

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

    fn fetch(world: &'data World) -> FetchResult<Self::Item> {
        let resource = world.resources().get_guarded().map(Ref::leak);
        let resource = resource.map(marker::Resource::new);
        Ok(resource)
    }
}
