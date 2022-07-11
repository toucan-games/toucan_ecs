use crate::component::Component;
use crate::error::FetchResult;
use crate::fetch::FetchViewOne;
use crate::system::fetch::Fetch;
use crate::world::view::ViewOne;
use crate::World;

impl<'data, C> Fetch<'data> for FetchViewOne<C>
where
    C: Component,
{
    type Item = ViewOne<'data, C>;

    unsafe fn fetch(world: *mut World) -> FetchResult<Self::Item> {
        Ok(Self::fetch(world))
    }
}
