use std::marker::PhantomData;

use crate::system::fetch::Fetch;
use crate::world::query::QueryMut;
use crate::world::view::ViewMut;

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
}
