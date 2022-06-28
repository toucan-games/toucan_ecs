use std::iter::Flatten;
use std::option::IntoIter;

use crate::component::{Component, IterMut, StorageImpl};
use crate::Entity;

/// Iterator which returns *entity* of the world
/// with **unique** *borrow* of component attached to it.
///
/// Only entities that has generic component type will be returned.
#[repr(transparent)]
pub struct ViewOneMut<'data, C>
where
    C: Component,
{
    iter: Flatten<IntoIter<IterMut<'data, C>>>,
}

impl<'data, C> ViewOneMut<'data, C>
where
    C: Component,
{
    pub(crate) fn new(storage: Option<&'data mut StorageImpl<C>>) -> Self {
        let iter = storage.map(StorageImpl::iter_mut).into_iter().flatten();
        Self { iter }
    }
}

impl<'data, C> Iterator for ViewOneMut<'data, C>
where
    C: Component,
{
    type Item = (Entity, &'data mut C);

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next()
    }
}
