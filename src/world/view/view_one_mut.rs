use crate::component::{Component, IterMut, StorageHolderMut};
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
    iter: Option<Box<IterMut<'data, C>>>,
}

impl<'data, C> ViewOneMut<'data, C>
where
    C: Component,
{
    pub(crate) fn new(storage: Option<StorageHolderMut<'data, C>>) -> Self {
        let iter = storage.map(StorageHolderMut::iter_mut);
        Self { iter }
    }
}

impl<'data, C> Iterator for ViewOneMut<'data, C>
where
    C: Component,
{
    type Item = (Entity, &'data mut C);

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.as_mut()?.next()
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let len = self.len();
        (len, Some(len))
    }
}

impl<'data, C> ExactSizeIterator for ViewOneMut<'data, C>
where
    C: Component,
{
    fn len(&self) -> usize {
        self.iter.as_ref().map(ExactSizeIterator::len).unwrap_or(0)
    }
}
