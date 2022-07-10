use crate::component::{Component, Iter, StorageHolder};
use crate::Entity;

/// Iterator which returns *entity* of the world
/// with **shared** *borrow* of component attached to it.
///
/// Only entities that has generic component type will be returned.
#[repr(transparent)]
pub struct ViewOne<'data, C>
where
    C: Component,
{
    iter: Option<Box<Iter<'data, C>>>,
}

impl<'data, C> ViewOne<'data, C>
where
    C: Component,
{
    pub(crate) fn new(storage: Option<StorageHolder<'data, C>>) -> Self {
        let iter = storage.as_ref().map(StorageHolder::iter);
        Self { iter }
    }
}

impl<'data, C> Iterator for ViewOne<'data, C>
where
    C: Component,
{
    type Item = (Entity, &'data C);

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.as_mut()?.next()
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let len = self.len();
        (len, Some(len))
    }
}

impl<'data, C> ExactSizeIterator for ViewOne<'data, C>
where
    C: Component,
{
    fn len(&self) -> usize {
        self.iter.as_ref().map(ExactSizeIterator::len).unwrap_or(0)
    }
}
