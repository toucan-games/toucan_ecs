use slotmap::dense::Keys;

use crate::component::{Component, DefaultStorage, Registry};
use crate::Entity;

/// Iterator which returns [entities][`Entity`] and their shared borrows of components.
///
/// Only entities that has generic component type will be returned.
pub struct ViewOne<'data, C>
where
    C: Component,
{
    entities: Keys<'data, Entity, ()>,
    storage: Option<&'data DefaultStorage<C>>,
}

impl<'data, C> ViewOne<'data, C>
where
    C: Component,
{
    pub(super) fn new(registry: &'data Registry) -> Self {
        let entities = registry.entities();
        let storage = registry.get_storage();
        Self { entities, storage }
    }
}

impl<'data, C> Iterator for ViewOne<'data, C>
where
    C: Component,
{
    type Item = &'data C;

    fn next(&mut self) -> Option<Self::Item> {
        let storage = self.storage?;
        loop {
            let entity = self.entities.next()?;
            if let Some(component) = storage.get(entity) {
                return Some(component);
            }
        }
    }
}