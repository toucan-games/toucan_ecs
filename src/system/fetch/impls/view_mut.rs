use crate::error::FetchResult;
use crate::fetch::FetchViewMut;
use crate::system::fetch::Fetch;
use crate::world::query::QueryMut;
use crate::world::view::ViewMut;
use crate::world::World;

impl<'data, Q> Fetch<'data> for FetchViewMut<'data, Q>
where
    Q: QueryMut<'data>,
{
    type Item = ViewMut<'data, Q>;

    unsafe fn fetch(world: *mut World) -> FetchResult<Self::Item> {
        Ok(Self::fetch(world))
    }
}
