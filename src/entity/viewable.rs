use crate::world::{SharedViewable, Viewable};
use crate::{Component, Entity};

use super::fetch::{FetchEntity, FetchOptionRead, FetchOptionWrite, FetchRead, FetchWrite};

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
