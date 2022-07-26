use std::mem::transmute;

use crate::entity;
use crate::system::foreach::fetch::{find_optimal, Fetch, FetchData, FetchStrategy};
use crate::world::query::QueryMut;
use crate::world::World;

/// Iterator which returns **unique** borrows of components.
///
/// It will be constructed from the query which is determined by the generic type.
/// Only entities that satisfy the query will be returned.
///
/// List of available types to query is located in [`world::query`](crate::world::query) module.
// TODO: turn into the lending iterator because
//  resources' mutable references could be copied freely
pub struct ViewMut<'data, Q>
where
    Q: QueryMut<'data>,
{
    entities: entity::Iter<'data>,
    fetch: Option<Q::Fetch>,
}

impl<'data, Q> ViewMut<'data, Q>
where
    Q: QueryMut<'data>,
{
    // noinspection RsUnnecessaryQualifications, DuplicatedCode
    pub(crate) fn new(world: &'data mut World, undo_leak: bool) -> Self {
        if undo_leak {
            world.components_mut().undo_leak();
        }
        let (entities, data) = world.split();
        let optimal = find_optimal::<'data, Q::Fetch>(data).map(FetchData::into_type_id);
        let fetch = Q::Fetch::new(data, optimal).ok();
        let entities = entities.iter();
        Self { entities, fetch }
    }
}

impl<'data, Q> Iterator for ViewMut<'data, Q>
where
    Q: QueryMut<'data>,
{
    type Item = Q;

    // noinspection DuplicatedCode
    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let fetch: &'data mut Q::Fetch = unsafe { transmute(self.fetch.as_mut()?) };
            let entities: &'data mut entity::Iter = unsafe { transmute(&mut self.entities) };
            let strategy = fetch
                .is_iter()
                .then_some(FetchStrategy::Optimized)
                .unwrap_or_else(|| FetchStrategy::All(entities));
            let result = fetch.fetch_iter(strategy);
            match result {
                Ok(item) => {
                    let (_, item) = item?;
                    return Some(item.into());
                }
                Err(_) => continue,
            }
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let upper = self.entities.len();
        (0, Some(upper))
    }
}
