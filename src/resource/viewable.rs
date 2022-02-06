use std::marker::PhantomData;

use crate::world::{SharedViewable, Viewable};
use crate::Resource;

use super::{
    fetch::{FetchRead, FetchWrite},
    Ref, RefMut,
};

/// Marker for retrieving shared borrow of resource from the world.
/// It must be used in query to be retrieved.
pub struct ResourceRead<'data, R>
where
    R: Resource,
{
    _ph: PhantomData<Ref<'data, R>>,
}

impl<'data, R> Viewable<'data> for ResourceRead<'data, R>
where
    R: Resource,
{
    type Fetch = FetchRead<'data, R>;
}

impl<'data, R> SharedViewable<'data> for ResourceRead<'data, R> where R: Resource {}

/// Marker for retrieving unique borrow of resource from the world.
/// It must be used in query to be retrieved.
pub struct ResourceWrite<'data, R>
where
    R: Resource,
{
    _ph: PhantomData<RefMut<'data, R>>,
}

impl<'data, R> Viewable<'data> for ResourceWrite<'data, R>
where
    R: Resource,
{
    type Fetch = FetchWrite<'data, R>;
}
