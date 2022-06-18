use std::iter::Flatten;
use std::option::IntoIter;

use crate::component::{Component, Iter, IterMut, Registry, StorageImpl};
use crate::Entity;

/// Iterator which returns shared borrows of components.
///
/// Only entities that has generic component type will be returned.
#[repr(transparent)]
pub struct ViewOne<'data, C>
where
    C: Component,
{
    iter: Flatten<IntoIter<Iter<'data, C>>>,
}

impl<'data, C> ViewOne<'data, C>
where
    C: Component,
{
    pub(super) fn new(registry: &'data Registry) -> Self {
        let iter = registry
            .get_storage()
            .map(StorageImpl::iter)
            .into_iter()
            .flatten();
        Self { iter }
    }
}

impl<'data, C> Iterator for ViewOne<'data, C>
where
    C: Component,
{
    type Item = (Entity, &'data C);

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next()
    }
}

/// Iterator which returns unique borrows of components.
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
    pub(super) fn new(registry: &'data mut Registry) -> Self {
        let iter = registry
            .get_storage_mut()
            .map(StorageImpl::iter_mut)
            .into_iter()
            .flatten();
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
