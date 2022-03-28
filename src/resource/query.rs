use crate::world::{Query, QueryShared};

use super::fetch::{FetchRead, FetchWrite};
use super::marker::Resource as ResourceMarker;
use super::Resource;

impl<'data, R> Query<'data> for ResourceMarker<&'data R>
where
    R: Resource,
{
    type Fetch = FetchRead<'data, R>;
}

impl<'data, R> QueryShared<'data> for ResourceMarker<&'data R> where R: Resource {}

impl<'data, R> Query<'data> for ResourceMarker<&'data mut R>
where
    R: Resource,
{
    type Fetch = FetchWrite<'data, R>;
}
