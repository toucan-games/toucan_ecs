//! Special marker types for views and systems.

use std::marker::PhantomData;

/// Marker for retrieving shared/unique borrow of resource from the world.
/// It must be used in query to be retrieved.
pub struct Resource<R>(PhantomData<R>);
