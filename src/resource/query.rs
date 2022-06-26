use crate::world::query::{Query, QueryMut};

use super::{marker, Resource};
use super::fetch::{FetchRead, FetchReadMut, FetchWriteMut};

impl<'data, R> Query<'data> for marker::Resource<'data, R>
where
    R: Resource,
{
    type Fetch = FetchRead<'data, R>;
}

impl<'data, R> QueryMut<'data> for marker::Resource<'data, R>
where
    R: Resource,
{
    type Fetch = FetchReadMut<'data, R>;
}

impl<'data, R> QueryMut<'data> for marker::ResourceMut<'data, R>
where
    R: Resource,
{
    type Fetch = FetchWriteMut<'data, R>;
}
