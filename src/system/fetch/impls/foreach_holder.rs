use crate::error::FetchResult;
use crate::fetch::FetchForeachHolder;
use crate::system::fetch::Fetch;
use crate::system::foreach::{ForeachHolder, Query};
use crate::World;

impl<'data, Q> Fetch<'data> for FetchForeachHolder<'data, Q>
where
    Q: Query<'data>,
{
    type Item = ForeachHolder<'data, Q>;

    unsafe fn fetch(world: *mut World) -> FetchResult<Self::Item> {
        Ok(Self::fetch(world))
    }
}
