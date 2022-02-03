use crate::entity::view::fetch::{FetchOptionRead, FetchOptionWrite, FetchRead, FetchWrite};
use crate::Component;

use super::{SharedViewable, Viewable};

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
