use std::marker::PhantomData;

use crate::world::{SharedViewable, Viewable};
use crate::Resource;

use super::fetch::{FetchRead, FetchWrite};

/// Marker for retrieving shared/unique borrow of resource from the world.
/// It must be used in query to be retrieved.
pub struct Res<R>(PhantomData<*const R>);

impl<'data, R> Viewable<'data> for Res<&'data R>
where
    R: Resource,
{
    type Fetch = FetchRead<'data, R>;
}

impl<'data, R> SharedViewable<'data> for Res<&'data R> where R: Resource {}

impl<'data, R> Viewable<'data> for Res<&'data mut R>
where
    R: Resource,
{
    type Fetch = FetchWrite<'data, R>;
}
