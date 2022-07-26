use std::collections::HashMap;
use std::hash::BuildHasherDefault;

use atomicell::{AtomicCell, Ref, RefMut};

use crate::component::storage::{ErasedStorageHolder, Storage};
use crate::component::{Component, ComponentSet, ComponentTypeId};
use crate::entity::Entity;
use crate::hash::TypeIdHasher;

type StorageRefCell = AtomicCell<ErasedStorageHolder>;

#[derive(Default)]
#[repr(transparent)]
pub struct Registry {
    storages: HashMap<ComponentTypeId, StorageRefCell, BuildHasherDefault<TypeIdHasher>>,
}

impl Registry {
    pub fn clear(&mut self) {
        self.storages
            .values_mut()
            .map(AtomicCell::get_mut)
            .for_each(ErasedStorageHolder::clear);
    }

    pub fn register<C>(&mut self)
    where
        C: Component,
    {
        if !self.has_storage::<C>() {
            self.create_storage::<C>();
        }
    }

    pub(super) fn attach_one<C>(&mut self, entity: Entity, component: C)
    where
        C: Component,
    {
        self.register::<C>();
        let storage: &mut C::Storage = self.get_storage_mut::<C>().unwrap();
        storage.attach(entity, component);
    }

    pub fn attach<S>(&mut self, entity: Entity, set: S)
    where
        S: ComponentSet,
    {
        set.attach(self, entity)
    }

    pub(super) fn attached_one<C>(&self, entity: Entity) -> bool
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
            .map(|it| unsafe {
                it.try_borrow_unguarded()
                    .expect("storage was already borrowed as mutable")
            })
            .all(|storage| !storage.attached(entity))
    }

    pub(super) fn remove_one<C>(&mut self, entity: Entity)
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
            .map(AtomicCell::get_mut)
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

    pub fn get_storage<C>(&self) -> Option<&C::Storage>
    where
        C: Component,
    {
        let type_id = ComponentTypeId::of::<C>();
        let storage = self.storages.get(&type_id)?;
        let storage = unsafe { storage.try_borrow_unguarded() }
            .expect("storage was already borrowed as mutable");
        Some(storage.as_storage_ref())
    }

    pub(crate) fn get_storage_guarded<C>(&self) -> Option<Ref<C::Storage>>
    where
        C: Component,
    {
        let type_id = ComponentTypeId::of::<C>();
        let storage = self.storages.get(&type_id)?;
        let storage = Ref::map(storage.borrow(), ErasedStorageHolder::as_storage_ref);
        Some(storage)
    }

    pub fn get_storage_mut<C>(&mut self) -> Option<&mut C::Storage>
    where
        C: Component,
    {
        let type_id = ComponentTypeId::of::<C>();
        let storage = self.storages.get_mut(&type_id)?;
        Some(storage.get_mut().as_storage_mut())
    }

    pub(crate) fn get_storage_mut_guarded<C>(&self) -> Option<RefMut<C::Storage>>
    where
        C: Component,
    {
        let type_id = ComponentTypeId::of::<C>();
        let storage = self.storages.get(&type_id)?;
        let storage = RefMut::map(storage.borrow_mut(), ErasedStorageHolder::as_storage_mut);
        Some(storage)
    }

    pub fn has_storage<C>(&self) -> bool
    where
        C: Component,
    {
        let type_id = ComponentTypeId::of::<C>();
        self.storages.contains_key(&type_id)
    }

    // noinspection RsUnnecessaryQualifications
    fn create_storage<C>(&mut self)
    where
        C: Component,
    {
        let type_id = ComponentTypeId::of::<C>();
        let storage = C::Storage::default();
        let erased = storage.into();
        self.storages.insert(type_id, AtomicCell::new(erased));
    }

    pub(crate) fn undo_leak(&mut self) {
        for storage in self.storages.values_mut() {
            storage.undo_leak();
        }
    }
}
