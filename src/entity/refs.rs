use std::ops::{Deref, DerefMut};
use std::sync::MutexGuard;

use crate::Component;

/// Shared borrow of the entity's component.
#[repr(transparent)]
pub struct Ref<'data, C: Component>(pub(crate) MutexGuard<'data, C>);

impl<'data, C: Component> Deref for Ref<'data, C> {
    type Target = C;

    fn deref(&self) -> &Self::Target {
        self.0.deref()
    }
}

/// Unique borrow of the entity's component.
#[repr(transparent)]
pub struct RefMut<'data, C: Component>(pub(crate) MutexGuard<'data, C>);

impl<'data, C: Component> Deref for RefMut<'data, C> {
    type Target = C;

    fn deref(&self) -> &Self::Target {
        self.0.deref()
    }
}

impl<'data, C: Component> DerefMut for RefMut<'data, C> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.0.deref_mut()
    }
}
