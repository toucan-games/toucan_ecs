use std::ops::{Deref, DerefMut};
use std::sync::MutexGuard;

use crate::Component;

/// Shared borrow of the entity's component.
#[repr(transparent)]
pub struct Ref<'data, C: Component> {
    #[allow(non_snake_case)]
    __: MutexGuard<'data, C>,
}

impl<'data, C: Component> Ref<'data, C> {
    pub(crate) fn new(data: MutexGuard<'data, C>) -> Self {
        Self { __: data }
    }
}

impl<'data, C: Component> Deref for Ref<'data, C> {
    type Target = C;

    fn deref(&self) -> &Self::Target {
        self.__.deref()
    }
}

/// Unique borrow of the entity's component.
#[repr(transparent)]
pub struct RefMut<'data, C: Component> {
    #[allow(non_snake_case)]
    __: MutexGuard<'data, C>,
}

impl<'data, C: Component> RefMut<'data, C> {
    pub(crate) fn new(data: MutexGuard<'data, C>) -> Self {
        Self { __: data }
    }
}

impl<'data, C: Component> Deref for RefMut<'data, C> {
    type Target = C;

    fn deref(&self) -> &Self::Target {
        self.__.deref()
    }
}

impl<'data, C: Component> DerefMut for RefMut<'data, C> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.__.deref_mut()
    }
}
