use std::mem::transmute;

use crate::entity;
use crate::system::foreach::fetch::{find_optimal, Fetch, FetchData, FetchStrategy};
use crate::system::foreach::Query;
use crate::world::World;

pub struct ForeachHolder<'data, Q>
where
    Q: Query<'data>,
{
    entities: entity::Iter<'data>,
    fetch: Option<Q::Fetch>,
}

impl<'data, Q> ForeachHolder<'data, Q>
where
    Q: Query<'data>,
{
    // noinspection RsUnnecessaryQualifications, DuplicatedCode
    pub(crate) fn new(world: &'data mut World) -> Self {
        let (entities, data) = world.split();
        let optimal = find_optimal::<'data, Q::Fetch>(data).map(FetchData::into_type_id);
        let fetch = Q::Fetch::new(data, optimal).ok();
        let entities = entities.iter();
        Self { entities, fetch }
    }
}

impl<'data, Q> Iterator for ForeachHolder<'data, Q>
where
    Q: Query<'data>,
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
