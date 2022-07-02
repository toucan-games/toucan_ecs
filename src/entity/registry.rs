use std::iter::FusedIterator;

use slotmap::dense::Keys;
use slotmap::DenseSlotMap;

use super::Entity;

#[repr(transparent)]
#[derive(Default)]
pub struct Registry(DenseSlotMap<Entity, ()>);

impl Registry {
    pub fn create(&mut self) -> Entity {
        self.0.insert(())
    }

    pub fn contains(&self, entity: Entity) -> bool {
        self.0.contains_key(entity)
    }

    pub fn destroy(&mut self, entity: Entity) {
        self.0.remove(entity);
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    pub fn clear(&mut self) {
        self.0.clear();
    }

    pub fn iter(&self) -> Iter {
        Iter(self.0.keys())
    }
}

#[repr(transparent)]
pub struct Iter<'data>(Keys<'data, Entity, ()>);

impl<'data> Iterator for Iter<'data> {
    type Item = Entity;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.next()
    }
}

impl<'data> FusedIterator for Iter<'data> {}

impl<'data> ExactSizeIterator for Iter<'data> {}
