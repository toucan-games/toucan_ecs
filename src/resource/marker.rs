//! Special marker types for views and systems.

use std::ops::{Deref, DerefMut};

/// Marker for retrieving shared/unique borrow of resource from the world.
/// It must be used in query to be retrieved.
pub struct Resource<R> {
    data: R,
}

impl<R> Deref for Resource<R> {
    type Target = R;

    fn deref(&self) -> &Self::Target {
        &self.data
    }
}

impl<R> DerefMut for Resource<R> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.data
    }
}
