use slotmap::dense::{Iter, IterMut};
use slotmap::{DenseSlotMap, SecondaryMap};

use crate::component::Component;
use crate::entity::Entity;

use super::Storage;

slotmap::new_key_type! {
    pub struct ComponentKey;
}

#[derive(Default)]
pub struct DefaultStorage<C>
where
    C: Component,
{
    components: DenseSlotMap<ComponentKey, C>,
    mapping: SecondaryMap<Entity, ComponentKey>,
}

impl<C> DefaultStorage<C>
where
    C: Component,
{
    pub fn iter_items(&self) -> Iter<ComponentKey, C> {
        self.components.iter()
    }

    pub fn iter_items_mut(&mut self) -> IterMut<ComponentKey, C> {
        self.components.iter_mut()
    }
}

impl<C> DefaultStorage<C>
where
    C: Component,
{
    pub fn new() -> Self {
        Self {
            components: DenseSlotMap::with_key(),
            mapping: SecondaryMap::new(),
        }
    }

    pub fn attach(&mut self, entity: Entity, component: C) {
        let component = self.components.insert(component);
        self.mapping.insert(entity, component);
    }

    pub fn get(&self, entity: Entity) -> Option<&C> {
        let key = self.mapping.get(entity)?;
        self.components.get(*key)
    }

    pub fn get_mut(&mut self, entity: Entity) -> Option<&mut C> {
        let key = self.mapping.get(entity)?;
        self.components.get_mut(*key)
    }
}

impl<C> Storage for DefaultStorage<C>
where
    C: Component,
{
    fn remove(&mut self, entity: Entity) {
        let component = self.mapping.remove(entity);
        if let Some(component) = component {
            self.components.remove(component);
        }
    }

    fn attached(&self, entity: Entity) -> bool {
        self.mapping.contains_key(entity)
    }

    fn clear(&mut self) {
        self.mapping.clear();
        self.components.clear();
    }
}
