use slotmap::dense::{Iter, IterMut};
use slotmap::{DenseSlotMap, SecondaryMap};

use crate::component::Component;
use crate::entity::Entity;

use super::Storage;

slotmap::new_key_type! {
    pub struct ComponentKey;
}

pub struct StorageImpl<C>
where
    C: Component,
{
    components: DenseSlotMap<ComponentKey, C>,
    mapping: SecondaryMap<Entity, ComponentKey>,
}

impl<C> StorageImpl<C>
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

impl<C> Default for StorageImpl<C>
where
    C: Component,
{
    fn default() -> Self {
        Self {
            components: DenseSlotMap::with_key(),
            mapping: SecondaryMap::new(),
        }
    }
}

impl<C> Storage for StorageImpl<C>
where
    C: Component,
{
    type Item = C;

    fn attach(&mut self, entity: Entity, component: Self::Item) {
        let component = self.components.insert(component);
        self.mapping.insert(entity, component);
    }

    fn attached(&self, entity: Entity) -> bool {
        self.mapping.contains_key(entity)
    }

    fn get(&self, entity: Entity) -> Option<&Self::Item> {
        let key = self.mapping.get(entity)?;
        self.components.get(*key)
    }

    fn get_mut(&mut self, entity: Entity) -> Option<&mut Self::Item> {
        let key = self.mapping.get(entity)?;
        self.components.get_mut(*key)
    }

    fn remove(&mut self, entity: Entity) {
        let component = self.mapping.remove(entity);
        if let Some(component) = component {
            self.components.remove(component);
        }
    }

    fn clear(&mut self) {
        self.mapping.clear();
        self.components.clear();
    }
}
