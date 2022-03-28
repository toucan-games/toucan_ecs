use crate::world::{Query, QueryShared};

use super::fetch::{FetchNot, FetchOptionRead, FetchOptionWrite, FetchRead, FetchWrite};
use super::marker::Not;
use super::Component;

impl<'data, C> Query<'data> for &'data C
where
    C: Component,
{
    type Fetch = FetchRead<'data, C>;
}

impl<'data, C> QueryShared<'data> for &'data C where C: Component {}

impl<'data, C> Query<'data> for &'data mut C
where
    C: Component,
{
    type Fetch = FetchWrite<'data, C>;
}

impl<'data, C> Query<'data> for Option<&'data C>
where
    C: Component,
{
    type Fetch = FetchOptionRead<'data, C>;
}

impl<'data, C> QueryShared<'data> for Option<&'data C> where C: Component {}

impl<'data, C> Query<'data> for Option<&'data mut C>
where
    C: Component,
{
    type Fetch = FetchOptionWrite<'data, C>;
}

impl<'data, C> Query<'data> for Not<'data, C>
where
    C: Component,
{
    type Fetch = FetchNot<'data, C>;
}

impl<'data, C> QueryShared<'data> for Not<'data, C> where C: Component {}
