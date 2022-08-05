use crate::entity::Iter;
use crate::system::foreach::ForeachHolder;
use crate::world::query::QueryMut;
use crate::world::WorldRefs;

/// Iterator which returns **unique** borrows of components.
///
/// It will be constructed from the query which is determined by the generic type.
/// Only entities that satisfy the query will be returned.
///
/// List of available types to query is located in [`world::query`](crate::world::query) module.
#[repr(transparent)]
pub struct ViewMut<'data, Q>
where
    Q: QueryMut<'data>,
{
    inner: ForeachHolder<'data, Q>,
}

impl<'data, Q> ViewMut<'data, Q>
where
    Q: QueryMut<'data>,
{
    pub(crate) fn new(entities: Iter<'data>, data: &mut WorldRefs<'data>) -> Self {
        let inner = ForeachHolder::new(Some(entities), data);
        Self { inner }
    }
}

impl<'data, Q> Iterator for ViewMut<'data, Q>
where
    Q: QueryMut<'data>,
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
