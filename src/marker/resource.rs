use std::ops::{Deref, DerefMut};

use crate::resource;

/// Marker for retrieving **shared** borrow of [resource] from the world.
/// It must be used in query to be retrieved.
///
/// This struct is just a wrapper around **shared** [reference] of the resource.
///
/// [resource]: super::Resource
/// [reference]: prim@reference
#[repr(transparent)]
#[cfg_attr(docsrs, doc(cfg(feature = "resource")))]
pub struct Resource<'data, R>(&'data R)
where
    R: resource::Resource;

impl<'data, R> Resource<'data, R>
where
    R: resource::Resource,
{
    pub(crate) fn new(resource: &'data R) -> Self {
        Self(resource)
    }
}

impl<'data, R> Deref for Resource<'data, R>
where
    R: resource::Resource,
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
#[cfg_attr(docsrs, doc(cfg(feature = "resource")))]
pub struct ResourceMut<'data, R>(&'data mut R)
where
    R: resource::Resource;

impl<'data, R> ResourceMut<'data, R>
where
    R: resource::Resource,
{
    pub(crate) fn new(resource: &'data mut R) -> Self {
        Self(resource)
    }
}

impl<'data, R> Deref for ResourceMut<'data, R>
where
    R: resource::Resource,
{
    type Target = R;

    fn deref(&self) -> &Self::Target {
        self.0
    }
}

impl<'data, R> DerefMut for ResourceMut<'data, R>
where
    R: resource::Resource,
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.0
    }
}
