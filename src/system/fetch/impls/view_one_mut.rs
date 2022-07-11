use crate::component::Component;
use crate::error::FetchResult;
use crate::fetch::FetchViewOneMut;
use crate::system::fetch::Fetch;
use crate::world::view::ViewOneMut;
use crate::World;

impl<'data, C> Fetch<'data> for FetchViewOneMut<C>
where
    C: Component,
{
    type Item = ViewOneMut<'data, C>;

    unsafe fn fetch(world: *mut World) -> FetchResult<Self::Item> {
        Ok(Self::fetch(world))
    }
}
