//! Special marker types for views and systems.

use std::ops::{Deref, DerefMut};

/// Marker for retrieving **shared** borrow of [resource] from the world.
/// It must be used in query to be retrieved.
///
/// This struct is just a wrapper around **shared** [reference] of the resource.
///
/// [resource]: super::Resource
/// [reference]: prim@reference
#[repr(transparent)]
pub struct Resource<'data, R>(&'data R)
where
    R: super::Resource;

impl<'data, R> Resource<'data, R>
where
    R: super::Resource,
{
    pub(crate) fn new(resource: &'data R) -> Self {
        Self(resource)
    }
}

impl<'data, R> Deref for Resource<'data, R>
where
    R: super::Resource,
{
    type Target = R;

    fn deref(&self) -> &Self::Target {
        self.0
    }
}

/// Marker for retrieving **unique** borrow of [resource] from the world.
/// It must be used in query to be retrieved.
///
/// This struct is just a wrapper around **mutable** [reference] of the resource.
///
/// [resource]: super::Resource
/// [reference]: prim@reference
#[repr(transparent)]
pub struct ResourceMut<'data, R>(&'data mut R)
where
    R: super::Resource;

impl<'data, R> ResourceMut<'data, R>
where
    R: super::Resource,
{
    pub(crate) fn new(resource: &'data mut R) -> Self {
        Self(resource)
    }
}

impl<'data, R> Deref for ResourceMut<'data, R>
where
    R: super::Resource,
{
    type Target = R;

    fn deref(&self) -> &Self::Target {
        self.0
    }
}

impl<'data, R> DerefMut for ResourceMut<'data, R>
where
    R: super::Resource,
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.0
    }
}
