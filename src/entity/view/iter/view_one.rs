use slotmap::dense::Keys;

use crate::component::pool::ComponentPool;
use crate::{Component, Entity, Ref, RefMut, Registry};

/// Iterator which returns [entities][`Entity`] and their [shared borrows][`Ref`]
/// of components.
///
/// Only entities that has that type of component will be returned.
pub struct ViewOne<'data, C>
where
    C: Component,
{
    entities: Keys<'data, Entity, ()>,
    pool: Option<&'data ComponentPool<C>>,
}

impl<'data, C> ViewOne<'data, C>
where
    C: Component,
{
    // noinspection DuplicatedCode
    pub(in crate::entity) fn new(registry: &'data Registry) -> Self {
        let entities = registry.entities();
        let pool = registry.get_pool();
        Self { entities, pool }
    }
}

impl<'data, C> Iterator for ViewOne<'data, C>
where
    C: Component,
{
    type Item = (Entity, Ref<'data, C>);

    fn next(&mut self) -> Option<Self::Item> {
        let pool = self.pool?;
        loop {
            let entity = self.entities.next()?;
            if let Some(component) = pool.get(entity) {
                return Some((entity, component));
            }
        }
    }
}

/// Iterator which returns [entities][`Entity`] and their [unique borrows][`RefMut`]
/// of components.
///
/// Only entities that has that type of component will be returned.
pub struct ViewOneMut<'data, C>
where
    C: Component,
{
    entities: Keys<'data, Entity, ()>,
    pool: Option<&'data ComponentPool<C>>,
}

impl<'data, C> ViewOneMut<'data, C>
where
    C: Component,
{
    // noinspection DuplicatedCode
    pub(in crate::entity) fn new(registry: &'data Registry) -> Self {
        let entities = registry.entities();
        let pool = registry.get_pool();
        Self { entities, pool }
    }
}

impl<'data, C> Iterator for ViewOneMut<'data, C>
where
    C: Component,
{
    type Item = (Entity, RefMut<'data, C>);

    fn next(&mut self) -> Option<Self::Item> {
        let pool = self.pool?;
        loop {
            let entity = self.entities.next()?;
            if let Some(component) = pool.get_mut(entity) {
                return Some((entity, component));
            }
        }
    }
}
