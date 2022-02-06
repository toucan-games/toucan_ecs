use std::ops::{Deref, DerefMut};
use std::sync::MutexGuard;

use super::Component;

/// Shared borrow of the entity's component.
#[repr(transparent)]
pub struct Ref<'data, C>
where
    C: Component,
{
    #[allow(non_snake_case)]
    __: MutexGuard<'data, C>,
}

impl<'data, C> Ref<'data, C>
where
    C: Component,
{
    pub(super) fn new(data: MutexGuard<'data, C>) -> Self {
        Self { __: data }
    }
}

impl<'data, C> Deref for Ref<'data, C>
where
    C: Component,
{
    type Target = C;

    fn deref(&self) -> &Self::Target {
        self.__.deref()
    }
}

/// Unique borrow of the entity's component.
#[repr(transparent)]
pub struct RefMut<'data, C>
where
    C: Component,
{
    #[allow(non_snake_case)]
    __: MutexGuard<'data, C>,
}

impl<'data, C> RefMut<'data, C>
where
    C: Component,
{
    pub(super) fn new(data: MutexGuard<'data, C>) -> Self {
        Self { __: data }
    }
}

impl<'data, C> Deref for RefMut<'data, C>
where
    C: Component,
{
    type Target = C;

    fn deref(&self) -> &Self::Target {
        self.__.deref()
    }
}

impl<'data, C> DerefMut for RefMut<'data, C>
where
    C: Component,
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.__.deref_mut()
    }
}
