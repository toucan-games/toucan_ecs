use std::mem::transmute;

use crate::world::query::CheckedQuery;
use crate::world::FetchMut;
use crate::{entity::Iter, World};

use super::query::{Query, QueryItem, QueryMut, QueryMutItem};
use super::Fetch;

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
    entities: Iter<'data>,
    fetch: Option<Q::Fetch>,
}

impl<'data, Q> View<'data, Q>
where
    Q: Query<'data>,
{
    // noinspection RsUnnecessaryQualifications
    pub(super) fn new(world: &'data World) -> Self {
        let (entities, data) = world.split();
        let entities = entities.iter();
        let fetch = Q::Fetch::new(data).ok();
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
    entities: Iter<'data>,
    fetch: Option<Q::Fetch>,
}

impl<'data, Q> ViewMut<'data, Q>
where
    Q: QueryMut<'data>,
{
    pub(super) fn new(world: &'data mut World, _checked: CheckedQuery<'data, Q>) -> Self {
        let (entities, data) = world.split_mut();
        let entities = entities.iter();
        // SAFETY: query was checked by `CheckedQuery`
        let fetch = unsafe { Q::Fetch::new(data) }.ok();
        Self { entities, fetch }
    }
}

impl<'data, Q> Iterator for ViewMut<'data, Q>
where
    Q: QueryMut<'data>,
{
    type Item = QueryMutItem<'data, Q>;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let entity = self.entities.next()?;
            // SAFETY: no GATs?
            let fetch = unsafe { transmute::<_, &'data mut Q::Fetch>(self.fetch.as_mut()?) };
            let result = fetch.fetch_mut(entity);
            match result {
                Ok(item) => return Some(item),
                Err(_) => continue,
            }
        }
    }
}
