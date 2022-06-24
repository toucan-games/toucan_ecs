//! Special marker types for views and systems.

use std::marker::PhantomData;

use crate::component::Component;

/// Marker for retrieving entities without component of generic type.
/// It must be used in query to be retrieved.
pub struct Not<C>(pub(super) PhantomData<C>)
where
    C: Component;
