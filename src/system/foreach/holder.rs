use std::mem::transmute;

use crate::entity::Entities;
use crate::system::foreach::fetch::Fetch;
use crate::system::foreach::CheckedQuery;
use crate::world::World;

use super::query::Query;

pub struct ForeachHolder<'data, Q>
where
    Q: Query<'data>,
{
    entities: Entities<'data>,
    fetch: Option<Q::Fetch>,
}

impl<'data, Q> ForeachHolder<'data, Q>
where
    Q: Query<'data>,
{
    // noinspection RsUnnecessaryQualifications
    pub(crate) fn new(world: &'data mut World) -> Self {
        let _checked = CheckedQuery::<'data, Q>::new();
        let (entities, data) = world.split_mut();
        // SAFETY: query was checked by `CheckedQuery`
        let fetch = unsafe { Q::Fetch::new(data) }.ok();
        let entities = fetch
            .as_ref()
            .and_then(Fetch::entities)
            .map(Entities::Optimized)
            .unwrap_or_else(|| Entities::All(entities.iter()));
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
            let entity = self.entities.next()?;
            // SAFETY: no GATs?
            let fetch = unsafe { transmute::<_, &'data mut Q::Fetch>(self.fetch.as_mut()?) };
            let result = fetch.fetch(entity);
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
