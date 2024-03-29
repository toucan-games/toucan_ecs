use std::iter::FusedIterator;

use slotmap::dense;
use slotmap::{DenseSlotMap, SecondaryMap};

use crate::component::storage;
use crate::component::Component;
use crate::entity::Entity;

use super::Storage;

slotmap::new_key_type! {
    struct ComponentKey;
}

/// Default type of storage which can store any type of component
/// efficient enough for usual use cases.
pub struct DefaultStorage<C>
where
    C: Component,
{
    components: DenseSlotMap<ComponentKey, (Entity, C)>,
    entity_to_key: SecondaryMap<Entity, ComponentKey>,
}

impl<C> Default for DefaultStorage<C>
where
    C: Component,
{
    fn default() -> Self {
        Self {
            components: DenseSlotMap::with_key(),
            entity_to_key: SecondaryMap::new(),
        }
    }
}

impl<C> Storage for DefaultStorage<C>
where
    C: Component,
{
    type Item = C;

    fn attach(&mut self, entity: Entity, component: Self::Item) {
        let component = self.components.insert((entity, component));
        self.entity_to_key.insert(entity, component);
    }

    fn attached(&self, entity: Entity) -> bool {
        self.entity_to_key.contains_key(entity)
    }

    fn get(&self, entity: Entity) -> Option<&Self::Item> {
        let key = self.entity_to_key.get(entity)?;
        let (_, component) = self.components.get(*key)?;
        Some(component)
    }

    fn get_mut(&mut self, entity: Entity) -> Option<&mut Self::Item> {
        let key = self.entity_to_key.get(entity)?;
        let (_, component) = self.components.get_mut(*key)?;
        Some(component)
    }

    fn remove(&mut self, entity: Entity) {
        let component = self.entity_to_key.remove(entity);
        if let Some(component) = component {
            self.components.remove(component);
        }
    }

    fn clear(&mut self) {
        self.entity_to_key.clear();
        self.components.clear();
    }

    fn iter(&self) -> Box<storage::DynIter<Self::Item>> {
        let iter = self.components.iter();
        let iter = Iter { iter };
        Box::new(iter)
    }

    fn iter_mut(&mut self) -> Box<storage::DynIterMut<Self::Item>> {
        let iter_mut = self.components.iter_mut();
        let iter_mut = IterMut { iter_mut };
        Box::new(iter_mut)
    }
}

pub struct Iter<'data, C>
where
    C: Component,
{
    iter: dense::Iter<'data, ComponentKey, (Entity, C)>,
}

impl<'data, C> Iterator for Iter<'data, C>
where
    C: Component,
{
    type Item = (Entity, &'data C);

    // noinspection DuplicatedCode
    fn next(&mut self) -> Option<Self::Item> {
        self.iter
            .next()
            .map(|(_, (entity, component))| (*entity, component))
    }
}

impl<'data, C> FusedIterator for Iter<'data, C> where C: Component {}

impl<'data, C> ExactSizeIterator for Iter<'data, C>
where
    C: Component,
{
    fn len(&self) -> usize {
        self.iter.len()
    }
}

pub struct IterMut<'data, C>
where
    C: Component,
{
    iter_mut: dense::IterMut<'data, ComponentKey, (Entity, C)>,
}

impl<'data, C> Iterator for IterMut<'data, C>
where
    C: Component,
{
    type Item = (Entity, &'data mut C);

    // noinspection DuplicatedCode
    fn next(&mut self) -> Option<Self::Item> {
        self.iter_mut
            .next()
            .map(|(_, (entity, component))| (*entity, component))
    }
}

impl<'data, C> FusedIterator for IterMut<'data, C> where C: Component {}

impl<'data, C> ExactSizeIterator for IterMut<'data, C>
where
    C: Component,
{
    fn len(&self) -> usize {
        self.iter_mut.len()
    }
}
