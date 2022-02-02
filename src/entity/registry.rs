use std::collections::HashMap;

use slotmap::DenseSlotMap;

use crate::component::{
    pool::{ComponentPool, Pool},
    set::ComponentSet,
    type_id::ComponentTypeId,
};
use crate::{Component, Entity, Entry, Ref, RefMut};

use super::view::{SharedViewable, View, ViewMut, ViewOne, ViewOneMut, Viewable};

pub struct Registry {
    entities: DenseSlotMap<Entity, ()>,
    pools: HashMap<ComponentTypeId, Box<dyn Pool>>,
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
        self.attach(entity, set);
        entity
    }

    pub fn create_entry(&mut self) -> Entry {
        let entity = self.create();
        Entry::new(entity, self)
    }

    pub fn entry(&mut self, entity: Entity) -> Option<Entry> {
        self.contains(entity).then(|| Entry::new(entity, self))
    }

    pub fn contains(&self, entity: Entity) -> bool {
        self.entities.contains_key(entity)
    }

    pub fn destroy(&mut self, entity: Entity) {
        self.remove_all(entity);
        self.entities.remove(entity);
    }

    pub fn attach_one<C>(&mut self, entity: Entity, component: C)
    where
        C: Component,
    {
        self.register::<C>();
        let pool = self.get_pool_mut().unwrap();
        pool.attach(entity, component);
    }

    pub fn attach<S>(&mut self, entity: Entity, set: S)
    where
        S: ComponentSet,
    {
        set.attach(self, entity)
    }

    pub fn attached_one<C>(&self, entity: Entity) -> bool
    where
        C: Component,
    {
        let pool = self.get_pool::<C>();
        pool.map(|pool| pool.attached(entity)).unwrap_or(false)
    }

    pub fn attached<S>(&self, entity: Entity) -> bool
    where
        S: ComponentSet,
    {
        S::attached(self, entity)
    }

    pub fn remove_one<C>(&mut self, entity: Entity)
    where
        C: Component,
    {
        let pool = self.get_pool_mut::<C>();
        if let Some(pool) = pool {
            pool.remove(entity)
        }
    }

    pub fn remove<S>(&mut self, entity: Entity)
    where
        S: ComponentSet,
    {
        S::remove(self, entity)
    }

    pub fn remove_all(&mut self, entity: Entity) {
        self.pools.values_mut().for_each(|pool| pool.remove(entity))
    }

    pub fn get<C>(&self, entity: Entity) -> Option<Ref<C>>
    where
        C: Component,
    {
        let pool = self.get_pool::<C>()?;
        pool.get(entity)
    }

    pub fn get_mut<C>(&mut self, entity: Entity) -> Option<RefMut<C>>
    where
        C: Component,
    {
        let pool = self.get_pool_mut::<C>()?;
        pool.get_mut(entity)
    }

    pub fn view_one<C>(&self) -> ViewOne<C>
    where
        C: Component,
    {
        ViewOne::new(self.entities.keys(), self.get_pool())
    }

    pub fn view_mut_one<C>(&mut self) -> ViewOneMut<C>
    where
        C: Component,
    {
        ViewOneMut::new(self.entities.keys(), self.get_pool())
    }

    pub fn view<'data, V>(&'data self) -> View<'data, V>
    where
        V: SharedViewable<'data>,
    {
        View::new(self.entities.keys(), self)
    }

    pub fn view_mut<'data, V>(&'data mut self) -> ViewMut<'data, V>
    where
        V: Viewable<'data>,
    {
        ViewMut::new(self.entities.keys(), self)
    }

    pub(super) fn get_pool<C>(&self) -> Option<&ComponentPool<C>>
    where
        C: Component,
    {
        let type_id = ComponentTypeId::of::<C>();
        let pool = self.pools.get(&type_id)?;
        let pool = pool
            .as_ref()
            .as_any_ref()
            .downcast_ref()
            .expect("downcast error");
        Some(pool)
    }

    fn get_pool_mut<C>(&mut self) -> Option<&mut ComponentPool<C>>
    where
        C: Component,
    {
        let type_id = ComponentTypeId::of::<C>();
        let pool = self.pools.get_mut(&type_id)?;
        let pool = pool
            .as_mut()
            .as_any_mut()
            .downcast_mut()
            .expect("downcast error");
        Some(pool)
    }

    fn create_pool<C>(&mut self)
    where
        C: Component,
    {
        let type_id = ComponentTypeId::of::<C>();
        let pool = ComponentPool::<C>::new();
        self.pools.insert(type_id, Box::new(pool));
    }
}
