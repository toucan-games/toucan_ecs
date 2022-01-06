use slotmap::dense::Keys;

use crate::component::pool::ComponentPool;
use crate::{Component, Entity};

pub struct View<'data, C>
where
    C: Component,
{
    entities: Keys<'data, Entity, ()>,
    pool: Option<&'data ComponentPool<C>>,
}

impl<'data, C> View<'data, C>
where
    C: Component,
{
    pub(super) fn new(
        entities: Keys<'data, Entity, ()>,
        pool: Option<&'data ComponentPool<C>>,
    ) -> Self {
        Self { entities, pool }
    }
}

impl<'data, C> Iterator for View<'data, C>
where
    C: Component,
{
    type Item = (Entity, &'data C);

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
