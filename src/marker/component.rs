use std::marker::PhantomData;

use crate::component::Component;

/// Marker for retrieving entities without component of generic type.
/// It must be used in query to be retrieved.
pub struct Not<C>(PhantomData<C>)
where
    C: Component;

impl<C> Not<C>
where
    C: Component,
{
    pub(crate) fn new() -> Self {
        Self(PhantomData)
    }
}
