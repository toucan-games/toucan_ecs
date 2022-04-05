use slotmap::secondary::{Iter, IterMut};
use slotmap::SecondaryMap;

use crate::component::Component;
use crate::entity::Entity;

use super::Storage;

#[derive(Default)]
#[repr(transparent)]
pub struct DefaultStorage<C>
where
    C: Component,
{
    components: SecondaryMap<Entity, C>,
}

impl<C> DefaultStorage<C>
where
    C: Component,
{
    pub fn iter(&self) -> Iter<Entity, C> {
        self.components.iter()
    }

    pub fn iter_mut(&mut self) -> IterMut<Entity, C> {
        self.components.iter_mut()
    }
}

impl<C> DefaultStorage<C>
where
    C: Component,
{
    pub fn new() -> Self {
        Self {
            components: SecondaryMap::new(),
        }
    }

    pub fn attach(&mut self, entity: Entity, component: C) {
        self.components.insert(entity, component);
    }

    pub fn get(&self, entity: Entity) -> Option<&C> {
        self.components.get(entity)
    }

    pub fn get_mut(&mut self, entity: Entity) -> Option<&mut C> {
        self.components.get_mut(entity)
    }
}

impl<C> Storage for DefaultStorage<C>
where
    C: Component,
{
    fn remove(&mut self, entity: Entity) {
        self.components.remove(entity);
    }

    fn attached(&self, entity: Entity) -> bool {
        self.components.contains_key(entity)
    }

    fn clear(&mut self) {
        self.components.clear();
    }
}
