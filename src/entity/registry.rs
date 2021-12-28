use std::any::{Any, TypeId};
use std::collections::HashMap;
use std::iter::empty;

use slotmap::SlotMap;

use crate::component::pool::ComponentPool;
use crate::entity::builder::EntityBuilder;
use crate::{Component, Entity};

pub struct Registry {
    entities: SlotMap<Entity, ()>,
    pools: HashMap<TypeId, Box<dyn Any>>,
}

impl Registry {
    pub fn new() -> Self {
        Self {
            entities: SlotMap::with_key(),
            pools: HashMap::new(),
        }
    }

    pub fn register<C>(&mut self)
    where
        C: Component,
    {
        let pool = self.get_pool_mut::<C>();
        if pool.is_none() {
            self.create_pool::<C>();
        }
    }

    pub fn create_entity(&mut self) -> Entity {
        self.entities.insert(())
    }

    pub fn build_entity(&mut self) -> EntityBuilder {
        EntityBuilder {
            entity: self.create_entity(),
            registry: self,
        }
    }

    pub fn attached(&self, entity: Entity) -> bool {
        self.entities.contains_key(entity)
    }

    pub fn remove(&mut self, entity: Entity) -> bool {
        self.entities.remove(entity).is_some()
    }

    pub fn attach<C>(&mut self, entity: Entity, component: C)
    where
        C: Component,
    {
        self.register::<C>();
        let pool = self.get_pool_mut().unwrap();
        pool.attach(entity, component);
    }

    pub fn view<C>(&self) -> impl Iterator<Item = (Entity, &C)>
    where
        C: Component,
    {
        // todo get view on set of components from tuple (by macros 😨)
        let pool = self
            .get_pool::<C>()
            .expect("component must be registered to be used");
        self.entities.iter().map(|(entity, _)| {
            let component = &pool[entity];
            (entity, component)
        })
    }

    fn get_pool<C>(&self) -> Option<&ComponentPool<C>>
    where
        C: Component,
    {
        let type_id = TypeId::of::<C>();
        let pool = self.pools.get(&type_id)?;
        let pool = pool.as_ref().downcast_ref().expect("downcast error");
        Some(pool)
    }

    fn get_pool_mut<C>(&mut self) -> Option<&mut ComponentPool<C>>
    where
        C: Component,
    {
        let type_id = TypeId::of::<C>();
        let pool = self.pools.get_mut(&type_id)?;
        let pool = pool.as_mut().downcast_mut().expect("downcast error");
        Some(pool)
    }

    fn create_pool<C>(&mut self) -> &mut ComponentPool<C>
    where
        C: Component,
    {
        let type_id = TypeId::of::<C>();
        let pool = ComponentPool::<C>::new();
        self.pools.insert(type_id, Box::new(pool));
        self.get_pool_mut().unwrap()
    }
}
