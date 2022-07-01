use crate::world::query::Query;
use crate::world::Fetch;
use crate::{Entity, World};

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
    entities: Box<dyn ExactSizeIterator<Item = Entity> + Send + Sync + 'data>,
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
            .unwrap_or_else(|| Box::new(entities.iter()));
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
        let entities = self.entities.as_mut();
        loop {
            let entity = entities.next()?;
            let result = fetch.fetch(entity);
            match result {
                Ok(item) => return Some(item.into()),
                Err(_) => continue,
            }
        }
    }
}
