use std::hash::Hash;
use std::ops::{Deref, DerefMut};

use crate::resource::Resource;

/// Marker for retrieving **shared** borrow of [resource] from the world.
/// It must be used in query to be retrieved.
///
/// This struct is just a wrapper around **shared** [reference] of the resource.
///
/// [resource]: Resource
/// [reference]: prim@reference
#[repr(transparent)]
#[cfg_attr(docsrs, doc(cfg(feature = "resource")))]
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Res<'data, R>(pub &'data R)
where
    R: Resource;

impl<'data, R> Deref for Res<'data, R>
where
    R: Resource,
{
    type Target = R;

    fn deref(&self) -> &Self::Target {
        self.0
    }
}

impl<'data, R> Clone for Res<'data, R>
where
    R: Resource,
{
    fn clone(&self) -> Self {
        Self(self.0)
    }
}

impl<'data, R> Copy for Res<'data, R> where R: Resource {}

impl<'data, R> From<&'data R> for Res<'data, R>
where
    R: Resource,
{
    fn from(shared: &'data R) -> Self {
        Self(shared)
    }
}

/// Marker for retrieving **unique** borrow of [resource] from the world.
/// It must be used in query to be retrieved.
///
/// This struct is just a wrapper around **mutable** [reference] of the resource.
///
/// [resource]: Resource
/// [reference]: prim@reference
#[repr(transparent)]
#[cfg_attr(docsrs, doc(cfg(feature = "resource")))]
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ResMut<'data, R>(pub &'data mut R)
where
    R: Resource;

impl<'data, R> Deref for ResMut<'data, R>
where
    R: Resource,
{
    type Target = R;

    fn deref(&self) -> &Self::Target {
        self.0
    }
}

impl<'data, R> DerefMut for ResMut<'data, R>
where
    R: Resource,
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.0
    }
}

impl<'data, R> From<&'data mut R> for ResMut<'data, R>
where
    R: Resource,
{
    fn from(unique: &'data mut R) -> Self {
        Self(unique)
    }
}
