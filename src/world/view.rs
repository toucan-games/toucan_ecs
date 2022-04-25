use crate::{entity::registry::Iter, World};

use super::{Fetch, Query, QueryItem};

/// Iterator which returns [entities][`Entity`] and their shared borrows of components.
///
/// It will be constructed from the query which is determined by the generic type.
/// Only entities that satisfies the query will be returned.
pub struct View<'data, Q>
where
    Q: Query<'data>,
{
    entities: Iter<'data>,
    fetch: Option<Q::Fetch>,
}

impl<'data, Q> View<'data, Q>
where
    Q: Query<'data>,
{
    pub(super) fn new(world: &'data World) -> Self {
        let entities = world.components().entities();
        let fetch = world.try_into().ok();
        Self { entities, fetch }
    }
}

impl<'data, Q> Iterator for View<'data, Q>
where
    Q: Query<'data>,
{
    type Item = QueryItem<'data, Q>;

    fn next(&mut self) -> Option<Self::Item> {
        let fetch = self.fetch.as_ref()?;
        loop {
            let entity = self.entities.next()?;
            let result = fetch.fetch(entity);
            match result {
                Ok(item) => return Some(item),
                Err(_) => continue,
            }
        }
    }
}
