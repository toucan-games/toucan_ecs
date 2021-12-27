use slotmap::SlotMap;

use crate::{Component, Entity};

pub struct EntityManager {
    entities: SlotMap<Entity, ()>,
}

impl EntityManager {
    pub fn new() -> Self {
        Self {
            entities: SlotMap::with_key(),
        }
    }

    pub fn create(&mut self) -> Entity {
        self.entities.insert(())
    }

    pub fn create_with<C>(&mut self, component: C) -> Entity
    where
        C: Component,
    {
        let entity = self.entities.insert(());
        todo!("create entity with provided set of components from tuple (by macros 😨)");
        entity
    }

    pub fn contains(&self, entity: Entity) -> bool {
        self.entities.contains_key(entity)
    }

    pub fn remove(&mut self, entity: Entity) -> bool {
        self.entities.remove(entity).is_some()
    }

    pub fn insert<C>(&mut self, entity: Entity, component: C)
    where
        C: Component,
    {
        todo!("insert set of components from tuple (by macros 😨)")
    }

    pub fn view<C>(&self)
    where
        C: Component,
    {
        todo!("get view on set of components from tuple (by macros 😨)")
    }
}
