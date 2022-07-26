use std::marker::PhantomData;

use crate::error::FetchResult;
use crate::system::fetch::Fetch;
use crate::system::foreach::{ForeachHolder, Query};
use crate::world::World;

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

    fn fetch(world: &'data World) -> FetchResult<Self::Item> {
        let foreach_holder = ForeachHolder::new(world, false);
        Ok(foreach_holder)
    }
}
