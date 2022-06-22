//! Special marker types for views and systems.

use std::marker::PhantomData;
use std::ops::{Deref, DerefMut};

use super::Resource as ResourceTrait;

/// Marker for retrieving shared borrow of resource from the world.
/// It must be used in query to be retrieved.
#[repr(transparent)]
pub struct Resource<'data, R>
where
    R: ResourceTrait,
{
    resource: &'data R,
}

impl<'data, R> Resource<'data, R>
where
    R: ResourceTrait,
{
    pub(super) fn new(resource: &'data R) -> Self {
        Self { resource }
    }
}

impl<'data, R> Deref for Resource<'data, R>
where
    R: ResourceTrait,
{
    type Target = R;

    fn deref(&self) -> &Self::Target {
        self.resource
    }
}

/// Marker for retrieving unique borrow of resource from the world.
/// It must be used in query to be retrieved.
#[repr(transparent)]
pub struct ResourceMut<'data, R>
where
    R: ResourceTrait,
{
    resource: *mut R,
    _ph: PhantomData<&'data mut R>,
}

unsafe impl<'data, R> Send for ResourceMut<'data, R> where R: ResourceTrait {}

unsafe impl<'data, R> Sync for ResourceMut<'data, R> where R: ResourceTrait {}

impl<'data, R> ResourceMut<'data, R>
where
    R: ResourceTrait,
{
    /// # Safety
    ///
    /// Use this function if and only if soundness was checked earlier.
    pub(super) unsafe fn new(resource: *mut R) -> Self {
        Self { resource, _ph: PhantomData }
    }
}

impl<'data, R> Deref for ResourceMut<'data, R>
where
    R: ResourceTrait,
{
    type Target = R;

    fn deref(&self) -> &Self::Target {
        // SAFETY: was checked at marker creation.
        unsafe { &*self.resource }
    }
}

impl<'data, R> DerefMut for ResourceMut<'data, R>
where
    R: ResourceTrait,
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        // SAFETY: was checked at marker creation.
        unsafe { &mut *self.resource }
    }
}
