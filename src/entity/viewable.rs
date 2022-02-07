use std::marker::PhantomData;

use crate::world::{SharedViewable, Viewable};
use crate::{Component, Entity};

use super::fetch::{
    FetchEntity, FetchNot, FetchOptionRead, FetchOptionWrite, FetchRead, FetchWrite,
};

impl<'data> Viewable<'data> for Entity {
    type Fetch = FetchEntity;
}

impl<'data> SharedViewable<'data> for Entity {}

impl<'data, C> Viewable<'data> for &'data C
where
    C: Component,
{
    type Fetch = FetchRead<'data, C>;
}

impl<'data, C> SharedViewable<'data> for &'data C where C: Component {}

impl<'data, C> Viewable<'data> for &'data mut C
where
    C: Component,
{
    type Fetch = FetchWrite<'data, C>;
}

impl<'data, C> Viewable<'data> for Option<&'data C>
where
    C: Component,
{
    type Fetch = FetchOptionRead<'data, C>;
}

impl<'data, C> SharedViewable<'data> for Option<&'data C> where C: Component {}

impl<'data, C> Viewable<'data> for Option<&'data mut C>
where
    C: Component,
{
    type Fetch = FetchOptionWrite<'data, C>;
}

/// Marker for retrieving entities without component of generic type.
/// It must be used in query to be retrieved.
pub struct Not<C>
where
    C: Component,
{
    _ph: PhantomData<*const C>,
}

impl<'data, C> Viewable<'data> for Not<C>
where
    C: Component,
{
    type Fetch = FetchNot<'data, C>;
}

impl<'data, C> SharedViewable<'data> for Not<C> where C: Component {}
