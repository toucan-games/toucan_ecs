use std::marker::PhantomData;

use crate::error::FetchResult;
use crate::system::fetch::Fetch;
use crate::system::foreach::{CheckedQuery, ForeachHolder, Query};
use crate::World;

#[repr(transparent)]
pub struct FetchForeachHolder<'data, Q>
where
    Q: Query<'data>,
{
    _ph: PhantomData<&'data Q>,
}

impl<'data, Q> Fetch<'data> for FetchForeachHolder<'data, Q>
where
    Q: Query<'data>,
{
    type Item = ForeachHolder<'data, Q>;

    unsafe fn fetch(world: *mut World) -> FetchResult<Self::Item> {
        let world = &mut *world;
        let checked = CheckedQuery::new();
        let holder = ForeachHolder::new(world, checked);
        Ok(holder)
    }
}
