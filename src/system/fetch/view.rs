use std::marker::PhantomData;

use crate::world::{Query, View};

use super::Fetch;

pub struct FetchView<'data, Q>
where
    Q: Query<'data>,
{
    _ph: PhantomData<&'data Q>,
}

impl<'data, Q> Fetch<'data> for FetchView<'data, Q>
where
    Q: Query<'data>,
{
    type Item = View<'data, Q>;
}
