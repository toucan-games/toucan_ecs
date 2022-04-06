use std::iter::Flatten;
use std::option::IntoIter;

use slotmap::dense::{Iter, IterMut};

use crate::component::{storage::ComponentKey, Component, DefaultStorage, Registry};

/// Iterator which returns shared borrows of components.
///
/// Only entities that has generic component type will be returned.
pub struct ViewOne<'data, C>
where
    C: Component,
{
    iter: Flatten<IntoIter<Iter<'data, ComponentKey, C>>>,
}

impl<'data, C> ViewOne<'data, C>
where
    C: Component,
{
    pub(super) fn new(registry: &'data Registry) -> Self {
        let iter = registry
            .get_storage()
            .map(DefaultStorage::iter_items)
            .into_iter()
            .flatten();
        Self { iter }
    }
}

impl<'data, C> Iterator for ViewOne<'data, C>
where
    C: Component,
{
    type Item = &'data C;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().map(|tuple| tuple.1)
    }
}

/// Iterator which returns unique borrows of components.
///
/// Only entities that has generic component type will be returned.
pub struct ViewOneMut<'data, C>
where
    C: Component,
{
    iter: Flatten<IntoIter<IterMut<'data, ComponentKey, C>>>,
}

impl<'data, C> ViewOneMut<'data, C>
where
    C: Component,
{
    pub(super) fn new(registry: &'data mut Registry) -> Self {
        let iter = registry
            .get_storage_mut()
            .map(DefaultStorage::iter_items_mut)
            .into_iter()
            .flatten();
        Self { iter }
    }
}

impl<'data, C> Iterator for ViewOneMut<'data, C>
where
    C: Component,
{
    type Item = &'data mut C;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().map(|tuple| tuple.1)
    }
}
