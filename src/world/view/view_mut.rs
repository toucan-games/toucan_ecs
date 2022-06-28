use std::mem::transmute;

use crate::entity::Iter;
use crate::world::query::{CheckedQuery, QueryMut};
use crate::world::FetchMut;
use crate::World;

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
    pub(crate) fn new(world: &'data mut World, _checked: CheckedQuery<'data, Q>) -> Self {
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
    type Item = Q;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let entity = self.entities.next()?;
            // SAFETY: no GATs?
            let fetch = unsafe { transmute::<_, &'data mut Q::Fetch>(self.fetch.as_mut()?) };
            let result = fetch.fetch_mut(entity);
            match result {
                Ok(item) => return Some(item.into()),
                Err(_) => continue,
            }
        }
    }
}
