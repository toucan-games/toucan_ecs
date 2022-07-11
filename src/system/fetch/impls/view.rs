use crate::error::FetchResult;
use crate::fetch::FetchView;
use crate::system::fetch::Fetch;
use crate::world::query::Query;
use crate::world::view::View;
use crate::world::World;

impl<'data, Q> Fetch<'data> for FetchView<'data, Q>
where
    Q: Query<'data>,
{
    type Item = View<'data, Q>;

    unsafe fn fetch(world: *mut World) -> FetchResult<Self::Item> {
        Ok(Self::fetch(world))
    }
}
