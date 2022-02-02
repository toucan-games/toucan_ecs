use slotmap::dense::Keys;

use crate::{Entity, Registry};

use super::{fetch::Fetch, SharedViewable, Viewable, ViewableItem};

pub struct View<'data, V>
where
    V: SharedViewable<'data>,
{
    entities: Keys<'data, Entity, ()>,
    registry: &'data Registry,
    fetch: Option<V::Fetch>,
}

impl<'data, V> View<'data, V>
where
    V: SharedViewable<'data>,
{
    pub(in crate::entity) fn new(
        entities: Keys<'data, Entity, ()>,
        registry: &'data Registry,
    ) -> Self {
        Self {
            entities,
            registry,
            fetch: None,
        }
    }
}

impl<'data, V> Iterator for View<'data, V>
where
    V: SharedViewable<'data>,
{
    type Item = (Entity, ViewableItem<'data, V>);

    // noinspection DuplicatedCode
    fn next(&mut self) -> Option<Self::Item> {
        if self.fetch.is_none() {
            let new_fetch = V::Fetch::try_from(self.registry).ok()?;
            self.fetch = Some(new_fetch);
        }
        let fetch = self.fetch.as_ref().unwrap();
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
    registry: &'data Registry,
    fetch: Option<V::Fetch>,
}

impl<'data, V> ViewMut<'data, V>
where
    V: Viewable<'data>,
{
    pub(in crate::entity) fn new(
        entities: Keys<'data, Entity, ()>,
        registry: &'data Registry,
    ) -> Self {
        Self {
            entities,
            registry,
            fetch: None,
        }
    }
}

impl<'data, V> Iterator for ViewMut<'data, V>
where
    V: Viewable<'data>,
{
    type Item = (Entity, ViewableItem<'data, V>);

    // noinspection DuplicatedCode
    fn next(&mut self) -> Option<Self::Item> {
        if self.fetch.is_none() {
            let new_fetch = V::Fetch::try_from(self.registry).ok()?;
            self.fetch = Some(new_fetch);
        }
        let fetch = self.fetch.as_ref().unwrap();
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
