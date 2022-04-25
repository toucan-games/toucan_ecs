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
pub struct Iter<'a>(Keys<'a, Entity, ()>);

impl<'a> Iterator for Iter<'a> {
    type Item = Entity;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.next()
    }
}
