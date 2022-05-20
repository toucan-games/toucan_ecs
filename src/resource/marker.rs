//! Special marker types for views and systems.

use std::ops::{Deref, DerefMut};

/// Marker for retrieving shared/unique borrow of resource from the world.
/// It must be used in query to be retrieved.
#[repr(transparent)]
pub struct Resource<R> {
    resource: R,
}

impl<R> From<R> for Resource<R> {
    fn from(resource: R) -> Self {
        Self { resource }
    }
}

impl<R> Deref for Resource<R> {
    type Target = R;

    fn deref(&self) -> &Self::Target {
        &self.resource
    }
}

impl<R> DerefMut for Resource<R> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.resource
    }
}
