use std::marker::PhantomData;

use crate::system::fetch::Fetch;
use crate::world::query::{CheckedQuery, QueryMut};
use crate::world::view::ViewMut;
use crate::World;

pub struct FetchViewMut<'data, Q>
where
    Q: QueryMut<'data>,
{
    _ph: PhantomData<&'data Q>,
}

impl<'data, Q> Fetch<'data> for FetchViewMut<'data, Q>
where
    Q: QueryMut<'data>,
{
    type Item = ViewMut<'data, Q>;

    unsafe fn fetch(world: &'data mut World) -> Self::Item {
        ViewMut::new(world, CheckedQuery::new())
    }
}
