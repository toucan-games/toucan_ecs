use crate::world::{Query, QueryMut};

use super::fetch::{FetchRead, FetchReadMut, FetchWriteMut};
use super::marker::Resource as ResourceMarker;
use super::Resource;

impl<'data, R> Query<'data> for ResourceMarker<&'data R>
where
    R: Resource,
{
    type Fetch = FetchRead<'data, R>;
}

impl<'data, R> QueryMut<'data> for ResourceMarker<&'data R>
where
    R: Resource,
{
    type Fetch = FetchReadMut<'data, R>;
}

impl<'data, R> QueryMut<'data> for ResourceMarker<&'data mut R>
where
    R: Resource,
{
    type Fetch = FetchWriteMut<'data, R>;
}
