//! Special marker types for views and systems.

use std::marker::PhantomData;

/// Marker for retrieving entities without component of generic type.
/// It must be used in query to be retrieved.
pub struct Not<'data, C>(pub(super) PhantomData<&'data C>);
