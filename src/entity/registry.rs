use std::any::Any;
use std::collections::HashMap;

use slotmap::DenseSlotMap;

use crate::component::{pool::ComponentPool, set::ComponentSet, type_id::ComponentTypeId};
use crate::{Component, Entity, Entry};

pub struct Registry {
    entities: DenseSlotMap<Entity, ()>,
    pools: HashMap<ComponentTypeId, Box<dyn Any>>,
}

impl Registry {
    pub fn new() -> Self {
        Self {
            entities: DenseSlotMap::with_key(),
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

    pub fn create(&mut self) -> Entity {
        self.entities.insert(())
    }

    pub fn create_with<S>(&mut self, set: S) -> Entity
    where
        S: ComponentSet,
    {
        let entity = self.create();
        self.attach_set(entity, set);
        entity
    }

    pub fn create_entry(&mut self) -> Entry {
        let entity = self.create();
        self.entry(entity).unwrap()
    }

    pub fn entry(&mut self, entity: Entity) -> Option<Entry> {
        self.attached(entity).then(|| Entry::new(entity, self))
    }

    pub fn attached(&self, entity: Entity) -> bool {
        self.entities.contains_key(entity)
    }

    pub fn destroy(&mut self, entity: Entity) -> bool {
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

    pub fn attach_set<S>(&mut self, entity: Entity, set: S)
    where
        S: ComponentSet,
    {
        set.attach(self, entity)
    }

    pub fn remove<C>(&mut self, entity: Entity)
    where
        C: Component,
    {
        let pool = self.get_pool_mut::<C>();
        if let Some(pool) = pool {
            pool.remove(entity)
        }
    }

    pub fn remove_set<S>(&mut self, entity: Entity)
    where
        S: ComponentSet,
    {
        S::remove(self, entity)
    }

    pub fn get<C>(&self, entity: Entity) -> Option<&C>
    where
        C: Component,
    {
        let pool = self.get_pool::<C>()?;
        pool.get(entity)
    }

    pub fn get_mut<C>(&mut self, entity: Entity) -> Option<&mut C>
    where
        C: Component,
    {
        let pool = self.get_pool_mut::<C>()?;
        pool.get_mut(entity)
    }

    pub fn view<C>(&self) -> impl Iterator<Item = (Entity, &C)>
    where
        C: Component,
    {
        let pool = self.get_pool();
        let entities = self.entities.keys();
        pool.map(|pool| {
            entities.filter_map(|entity| {
                let component = pool.get(entity)?;
                Some((entity, component))
            })
        })
        .into_iter()
        .flatten()
    }

    pub fn view_mut<C>(&mut self) -> impl Iterator<Item = (Entity, &mut C)>
    where
        C: Component,
    {
        todo!("get mutable view on set of components");
        std::iter::empty()
    }

    fn get_pool<C>(&self) -> Option<&ComponentPool<C>>
    where
        C: Component,
    {
        let type_id = ComponentTypeId::of::<C>();
        let pool = self.pools.get(&type_id)?;
        let pool = pool.as_ref().downcast_ref().expect("downcast error");
        Some(pool)
    }

    fn get_pool_mut<C>(&mut self) -> Option<&mut ComponentPool<C>>
    where
        C: Component,
    {
        let type_id = ComponentTypeId::of::<C>();
        let pool = self.pools.get_mut(&type_id)?;
        let pool = pool.as_mut().downcast_mut().expect("downcast error");
        Some(pool)
    }

    fn create_pool<C>(&mut self) -> &mut ComponentPool<C>
    where
        C: Component,
    {
        let type_id = ComponentTypeId::of::<C>();
        let pool = ComponentPool::<C>::new();
        self.pools.insert(type_id, Box::new(pool));
        self.get_pool_mut().unwrap()
    }
}
