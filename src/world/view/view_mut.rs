use std::mem::transmute;

use crate::world::query::{CheckedQuery, QueryMut};
use crate::world::FetchMut;
use crate::{Entity, World};

/// Iterator which returns **shared** and/or **unique** borrows of components.
///
/// It will be constructed from the query which is determined by the generic type.
/// Only entities that satisfy the query will be returned.
///
/// List of available types to query is located in [`world::query`](crate::world::query) module.
pub struct ViewMut<'data, Q>
where
    Q: QueryMut<'data>,
{
    entities: Box<dyn ExactSizeIterator<Item = Entity> + Send + Sync + 'data>,
    fetch: Option<Q::Fetch>,
}

impl<'data, Q> ViewMut<'data, Q>
where
    Q: QueryMut<'data>,
{
    pub(crate) fn new(world: &'data mut World, _checked: CheckedQuery<'data, Q>) -> Self {
        let (entities, data) = world.split_mut();
        // SAFETY: query was checked by `CheckedQuery`
        let fetch = unsafe { Q::Fetch::new(data) }.ok();
        let entities = fetch
            .as_ref()
            .and_then(FetchMut::entities)
            .unwrap_or_else(|| Box::new(entities.iter()));
        Self { entities, fetch }
    }
}

impl<'data, Q> Iterator for ViewMut<'data, Q>
where
    Q: QueryMut<'data>,
{
    type Item = Q;

    fn next(&mut self) -> Option<Self::Item> {
        let entities = self.entities.as_mut();
        loop {
            let entity = entities.next()?;
            // SAFETY: no GATs?
            let fetch = unsafe { transmute::<_, &'data mut Q::Fetch>(self.fetch.as_mut()?) };
            let result = fetch.fetch_mut(entity);
            match result {
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
