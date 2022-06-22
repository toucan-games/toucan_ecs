use std::marker::PhantomData;

use crate::world::{QueryMut, ViewMut};

use super::Fetch;

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
