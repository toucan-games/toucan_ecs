use crate::system::foreach::ForeachHolder;
use crate::world::query::Query;
use crate::world::World;

/// Iterator which returns **shared** borrows of components.
///
/// It will be constructed from the query which is determined by the generic type.
/// Only entities that satisfy the query will be returned.
///
/// List of available types to query is located in [`world::query`](crate::world::query) module.
#[repr(transparent)]
pub struct View<'data, Q>
where
    Q: Query<'data>,
{
    inner: ForeachHolder<'data, Q>,
}

impl<'data, Q> View<'data, Q>
where
    Q: Query<'data>,
{
    pub(crate) fn new(world: &'data World, undo_leak: bool) -> Self {
        let inner = ForeachHolder::new(world, undo_leak);
        Self { inner }
    }
}

impl<'data, Q> Iterator for View<'data, Q>
where
    Q: Query<'data>,
{
    type Item = Q;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next()
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.inner.size_hint()
    }
}
