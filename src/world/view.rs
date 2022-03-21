use slotmap::dense::Keys;

use crate::{Entity, World};

use super::{Fetch, Viewable, ViewableItem};

/// Iterator which returns [entities][`Entity`] and their shared borrows of components.
///
/// It will be constructed from the query which is determined by the generic type.
/// Only entities that satisfies the query will be returned.
pub struct View<'data, V>
where
    V: Viewable<'data>,
{
    entities: Keys<'data, Entity, ()>,
    fetch: Option<V::Fetch>,
}

impl<'data, V> View<'data, V>
where
    V: Viewable<'data>,
{
    // noinspection DuplicatedCode
    pub(crate) fn new(world: &'data World) -> Self {
        let entities = world.registry().entities();
        let fetch = V::Fetch::try_from(world).ok();
        Self { entities, fetch }
    }
}

impl<'data, V> Iterator for View<'data, V>
where
    V: Viewable<'data>,
{
    type Item = ViewableItem<'data, V>;

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
