use std::mem::transmute;

use crate::entity::{Entity, Iter};
use crate::system::foreach::fetch::{find_optimal, Fetch, FetchData, FetchStrategy};
use crate::system::foreach::query::{CheckedQuery, Query};
use crate::world::WorldRefs;

// TODO: turn into the lending iterator
pub struct ForeachHolder<'data, Q>
where
    Q: Query<'data>,
{
    entities: Option<Iter<'data>>,
    fetch: Option<Q::Fetch>,
}

impl<'data, Q> ForeachHolder<'data, Q>
where
    Q: Query<'data>,
{
    // noinspection RsUnnecessaryQualifications
    pub(crate) fn new(entities: Option<Iter<'data>>, data: &mut WorldRefs<'data>) -> Self {
        let _checked = CheckedQuery::<'data, Q>::new();
        let optimal = find_optimal::<Q::Fetch>(data).map(FetchData::into_type_id);
        let fetch = Q::Fetch::new(data, optimal).ok();
        Self { entities, fetch }
    }
}

impl<'data, Q> Iterator for ForeachHolder<'data, Q>
where
    Q: Query<'data>,
{
    type Item = Q;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            // SAFETY: returned data is valid for `'data` lifetime and no not overlap
            let (fetch, entities) = unsafe {
                let fetch: &'data mut Q::Fetch = transmute(self.fetch.as_mut()?);
                let entities: Option<&'data mut Iter> = transmute(self.entities.as_mut());
                (fetch, entities)
            };
            let result = match entities {
                None => {
                    let entity = Entity::default();
                    fetch.fetch_entity(entity).map(|item| Some((entity, item)))
                }
                Some(entities) => {
                    let strategy = fetch
                        .is_iter()
                        .then_some(FetchStrategy::Optimized)
                        .unwrap_or(FetchStrategy::All(entities));
                    fetch.fetch_iter(strategy)
                }
            };
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
        let upper = self.entities.as_ref().map(ExactSizeIterator::len);
        (0, upper)
    }
}
