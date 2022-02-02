use crate::Component;

use super::{FetchRead, FetchWrite, SharedViewable, Viewable};

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
