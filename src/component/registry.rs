use std::collections::HashMap;
use std::hash::BuildHasherDefault;

use crate::component::storage::ErasedStorageHolder;
use crate::component::{Component, ComponentSet, ComponentTypeId, Storage, StorageImpl};
use crate::hash::TypeIdHasher;
use crate::Entity;

#[derive(Default)]
#[repr(transparent)]
pub struct Registry {
    storages: HashMap<ComponentTypeId, ErasedStorageHolder, BuildHasherDefault<TypeIdHasher>>,
}

impl Registry {
    pub fn clear(&mut self) {
        self.storages
            .values_mut()
            .for_each(|storage| storage.clear());
    }

    pub fn register<C>(&mut self)
    where
        C: Component,
    {
        if !self.has_storage::<C>() {
            self.create_storage::<C>();
        }
    }

    pub fn attach_one<C>(&mut self, entity: Entity, component: C)
    where
        C: Component,
    {
        self.register::<C>();
        let storage = self.get_storage_mut().unwrap();
        storage.attach(entity, component);
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
        let storage = self.get_storage::<C>();
        storage
            .map(|storage| storage.attached(entity))
            .unwrap_or(false)
    }

    pub fn attached<S>(&self, entity: Entity) -> bool
    where
        S: ComponentSet,
    {
        S::attached(self, entity)
    }

    pub fn is_entity_empty(&self, entity: Entity) -> bool {
        self.storages
            .values()
            .all(|storage| !storage.attached(entity))
    }

    pub fn remove_one<C>(&mut self, entity: Entity)
    where
        C: Component,
    {
        let storage = self.get_storage_mut::<C>();
        if let Some(storage) = storage {
            storage.remove(entity)
        }
    }

    pub fn remove<S>(&mut self, entity: Entity)
    where
        S: ComponentSet,
    {
        S::remove(self, entity)
    }

    pub fn remove_all(&mut self, entity: Entity) {
        self.storages
            .values_mut()
            .for_each(|storage| storage.remove(entity))
    }

    pub fn get<C>(&self, entity: Entity) -> Option<&C>
    where
        C: Component,
    {
        let storage = self.get_storage::<C>()?;
        storage.get(entity)
    }

    pub fn get_mut<C>(&mut self, entity: Entity) -> Option<&mut C>
    where
        C: Component,
    {
        let storage = self.get_storage_mut::<C>()?;
        storage.get_mut(entity)
    }

    pub fn get_storage<C>(&self) -> Option<&StorageImpl<C>>
    where
        C: Component,
    {
        let type_id = ComponentTypeId::of::<C>();
        let storage = self.storages.get(&type_id)?;
        let storage = storage.downcast_ref().expect("downcast error");
        Some(storage)
    }

    pub fn get_storage_mut<C>(&mut self) -> Option<&mut StorageImpl<C>>
    where
        C: Component,
    {
        let type_id = ComponentTypeId::of::<C>();
        let storage = self.storages.get_mut(&type_id)?;
        let storage = storage.downcast_mut().expect("downcast error");
        Some(storage)
    }

    pub fn has_storage<C>(&self) -> bool
    where
        C: Component,
    {
        let type_id = ComponentTypeId::of::<C>();
        self.storages.contains_key(&type_id)
    }

    fn create_storage<C>(&mut self)
    where
        C: Component,
    {
        let type_id = ComponentTypeId::of::<C>();
        let storage = StorageImpl::<C>::default();
        self.storages.insert(type_id, storage.into());
    }
}
