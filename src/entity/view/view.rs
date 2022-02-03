use slotmap::dense::Keys;

use crate::{Entity, Registry};

use super::{fetch::Fetch, SharedViewable, Viewable, ViewableItem};

pub struct View<'data, V>
where
    V: SharedViewable<'data>,
{
    entities: Keys<'data, Entity, ()>,
    fetch: Option<V::Fetch>,
}

impl<'data, V> View<'data, V>
where
    V: SharedViewable<'data>,
{
    // noinspection DuplicatedCode
    pub(in crate::entity) fn new(registry: &'data Registry) -> Self {
        let entities = registry.entities();
        let fetch = V::Fetch::try_from(registry).ok();
        Self { entities, fetch }
    }
}

impl<'data, V> Iterator for View<'data, V>
where
    V: SharedViewable<'data>,
{
    type Item = (Entity, ViewableItem<'data, V>);

    // noinspection DuplicatedCode
    fn next(&mut self) -> Option<Self::Item> {
        let fetch = self.fetch.as_ref()?;
        loop {
            let entity = self.entities.next()?;
            let result = fetch.fetch(entity);
            match result {
                Ok(item) => return Some((entity, item)),
                Err(_) => continue,
            }
        }
    }
}

pub struct ViewMut<'data, V>
where
    V: Viewable<'data>,
{
    entities: Keys<'data, Entity, ()>,
    fetch: Option<V::Fetch>,
}

impl<'data, V> ViewMut<'data, V>
where
    V: Viewable<'data>,
{
    // noinspection DuplicatedCode
    pub(in crate::entity) fn new(registry: &'data Registry) -> Self {
        let entities = registry.entities();
        let fetch = V::Fetch::try_from(registry).ok();
        Self { entities, fetch }
    }
}

impl<'data, V> Iterator for ViewMut<'data, V>
where
    V: Viewable<'data>,
{
    type Item = (Entity, ViewableItem<'data, V>);

    // noinspection DuplicatedCode
    fn next(&mut self) -> Option<Self::Item> {
        let fetch = self.fetch.as_ref()?;
        loop {
            let entity = self.entities.next()?;
            let result = fetch.fetch(entity);
            match result {
                Ok(item) => return Some((entity, item)),
                Err(_) => continue,
            }
        }
    }
}
