use slotmap::dense::{Iter as DenseIter, IterMut as DenseIterMut};
use slotmap::{DenseSlotMap, SecondaryMap};

use crate::component::Component;
use crate::entity::Entity;

use super::Storage;

slotmap::new_key_type! {
    struct ComponentKey;
}

pub struct StorageImpl<C>
where
    C: Component,
{
    components: DenseSlotMap<ComponentKey, C>,
    entity_to_key: SecondaryMap<Entity, ComponentKey>,
    key_to_entity: SecondaryMap<ComponentKey, Entity>,
}

impl<C> StorageImpl<C>
where
    C: Component,
{
    pub fn iter(&self) -> Iter<C> {
        Iter {
            iter: self.components.iter(),
            key_to_entity: &self.key_to_entity,
        }
    }

    pub fn iter_mut(&mut self) -> IterMut<C> {
        IterMut {
            iter: self.components.iter_mut(),
            key_to_entity: &self.key_to_entity,
        }
    }
}

impl<C> Default for StorageImpl<C>
where
    C: Component,
{
    fn default() -> Self {
        Self {
            components: DenseSlotMap::with_key(),
            entity_to_key: SecondaryMap::new(),
            key_to_entity: SecondaryMap::new(),
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
        self.entity_to_key.insert(entity, component);
        self.key_to_entity.insert(component, entity);
    }

    fn attached(&self, entity: Entity) -> bool {
        self.entity_to_key.contains_key(entity)
    }

    fn get(&self, entity: Entity) -> Option<&Self::Item> {
        let key = self.entity_to_key.get(entity)?;
        self.components.get(*key)
    }

    fn get_mut(&mut self, entity: Entity) -> Option<&mut Self::Item> {
        let key = self.entity_to_key.get(entity)?;
        self.components.get_mut(*key)
    }

    fn remove(&mut self, entity: Entity) {
        let component = self.entity_to_key.remove(entity);
        if let Some(component) = component {
            self.components.remove(component);
            self.key_to_entity.remove(component);
        }
    }

    fn clear(&mut self) {
        self.entity_to_key.clear();
        self.key_to_entity.clear();
        self.components.clear();
    }
}

pub struct Iter<'data, C>
where
    C: Component,
{
    iter: DenseIter<'data, ComponentKey, C>,
    key_to_entity: &'data SecondaryMap<ComponentKey, Entity>,
}

impl<'data, C> Iterator for Iter<'data, C>
where
    C: Component,
{
    type Item = (Entity, &'data C);

    fn next(&mut self) -> Option<Self::Item> {
        self.iter
            .next()
            .map(|(key, data)| (self.key_to_entity[key], data))
    }
}

pub struct IterMut<'data, C>
where
    C: Component,
{
    iter: DenseIterMut<'data, ComponentKey, C>,
    key_to_entity: &'data SecondaryMap<ComponentKey, Entity>,
}

impl<'data, C> Iterator for IterMut<'data, C>
where
    C: Component,
{
    type Item = (Entity, &'data mut C);

    fn next(&mut self) -> Option<Self::Item> {
        self.iter
            .next()
            .map(|(key, data)| (self.key_to_entity[key], data))
    }
}
