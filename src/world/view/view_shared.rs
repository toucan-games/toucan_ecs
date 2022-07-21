use crate::entity::Entities;
use crate::world::query::Query;
use crate::world::{Fetch, World};

/// Iterator which returns **shared** borrows of components.
///
/// It will be constructed from the query which is determined by the generic type.
/// Only entities that satisfy the query will be returned.
///
/// List of available types to query is located in [`world::query`](crate::world::query) module.
pub struct View<'data, Q>
where
    Q: Query<'data>,
{
    entities: Entities<'data>,
    fetch: Option<Q::Fetch>,
}

impl<'data, Q> View<'data, Q>
where
    Q: Query<'data>,
{
    // noinspection RsUnnecessaryQualifications
    pub(crate) fn new(world: &'data World) -> Self {
        let (entities, data) = world.split();
        let fetch = Q::Fetch::new(data).ok();
        let entities = fetch
            .as_ref()
            .and_then(Fetch::entities)
            .map(Entities::Optimized)
            .unwrap_or_else(|| Entities::All(entities.iter()));
        Self { entities, fetch }
    }
}

impl<'data, Q> Iterator for View<'data, Q>
where
    Q: Query<'data>,
{
    type Item = Q;

    fn next(&mut self) -> Option<Self::Item> {
        let fetch = self.fetch.as_ref()?;
        loop {
            let entity = self.entities.next()?;
            match fetch.fetch(entity) {
                Ok(item) => return Some(item.into()),
                Err(_) => continue,
            }
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let upper = self.entities.len();
        (0, Some(upper))
    }
}
