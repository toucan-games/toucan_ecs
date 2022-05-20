use crate::component::fetch::{
    FetchNotMut, FetchOptionReadMut, FetchOptionWriteMut, FetchReadMut, FetchWriteMut,
};
use crate::world::{Query, QueryMut};

use super::fetch::{FetchNot, FetchOptionRead, FetchRead};
use super::marker::Not;
use super::Component;

impl<'data, C> Query<'data> for &'data C
where
    C: Component,
{
    type Fetch = FetchRead<'data, C>;
}

impl<'data, C> Query<'data> for Option<&'data C>
where
    C: Component,
{
    type Fetch = FetchOptionRead<'data, C>;
}

impl<'data, C> Query<'data> for Not<'data, C>
where
    C: Component,
{
    type Fetch = FetchNot<'data, C>;
}

impl<'data, C> QueryMut<'data> for &'data C
where
    C: Component,
{
    type Fetch = FetchReadMut<'data, C>;
}

impl<'data, C> QueryMut<'data> for &'data mut C
where
    C: Component,
{
    type Fetch = FetchWriteMut<'data, C>;
}

impl<'data, C> QueryMut<'data> for Option<&'data C>
where
    C: Component,
{
    type Fetch = FetchOptionReadMut<'data, C>;
}

impl<'data, C> QueryMut<'data> for Option<&'data mut C>
where
    C: Component,
{
    type Fetch = FetchOptionWriteMut<'data, C>;
}

impl<'data, C> QueryMut<'data> for Not<'data, C>
where
    C: Component,
{
    type Fetch = FetchNotMut<'data, C>;
}
