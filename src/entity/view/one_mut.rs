use slotmap::dense::Keys;

use crate::component::pool::ComponentPool;
use crate::{Component, Entity, RefMut};

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
    pub(in crate::entity) fn new(
        entities: Keys<'data, Entity, ()>,
        pool: Option<&'data ComponentPool<C>>,
    ) -> Self {
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
