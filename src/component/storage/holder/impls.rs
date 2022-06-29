use std::marker::PhantomData;

use crate::component::{Component, ComponentTypeId, Iter, IterMut};
use crate::Entity;

use super::erased::{ErasedComponent, ErasedStorageHolder};

#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct StorageHolder<'data, C>
where
    C: Component,
{
    erased: &'data ErasedStorageHolder,
    _ph: PhantomData<C>,
}

impl<'data, C> StorageHolder<'data, C>
where
    C: Component,
{
    pub fn attached(&self, entity: Entity) -> bool {
        self.erased.attached(entity)
    }

    // noinspection DuplicatedCode
    pub fn get(&self, entity: Entity) -> Option<&'data C> {
        let erased = self.erased.get(entity)?;
        // SAFETY: was checked at creation
        let component = unsafe { &*(erased.get() as *const _) };
        Some(component)
    }

    // noinspection DuplicatedCode
    pub fn iter(self) -> Box<Iter<'data, C>> {
        let iter = self.erased.iter();
        let iter = iter.map(|it| {
            let entity = it.0;
            // SAFETY: pointer from the holdable iter is valid
            let component = unsafe { &*(it.1.get() as *const _) };
            (entity, component)
        });
        Box::new(iter)
    }
}

impl<'data, C> From<&'data ErasedStorageHolder> for StorageHolder<'data, C>
where
    C: Component,
{
    fn from(erased: &'data ErasedStorageHolder) -> Self {
        debug_assert_eq!(erased.type_id(), &ComponentTypeId::of::<C>());
        Self {
            erased,
            _ph: PhantomData,
        }
    }
}

#[repr(transparent)]
pub struct StorageHolderMut<'data, C>
where
    C: Component,
{
    erased: &'data mut ErasedStorageHolder,
    _ph: PhantomData<C>,
}

impl<'data, C> StorageHolderMut<'data, C>
where
    C: Component,
{
    pub fn attach(&mut self, entity: Entity, component: C) {
        // SAFETY: component reference cannot be null
        let component = unsafe { ErasedComponent::new_unchecked(&component as *const _ as _) };
        // SAFETY: was checked at creation
        unsafe { self.erased.attach(entity, component) }
    }

    pub fn attached(&self, entity: Entity) -> bool {
        self.erased.attached(entity)
    }

    // noinspection DuplicatedCode
    pub fn get(&self, entity: Entity) -> Option<&'data C> {
        let erased = self.erased.get(entity)?;
        // SAFETY: was checked at creation
        let component = unsafe { &*(erased.get() as *const _) };
        Some(component)
    }

    pub fn get_mut(&mut self, entity: Entity) -> Option<&'data mut C> {
        let erased = self.erased.get_mut(entity)?;
        // SAFETY: was checked at creation
        let component = unsafe { &mut *(erased.get() as *mut _) };
        Some(component)
    }

    pub fn remove(&mut self, entity: Entity) {
        self.erased.remove(entity)
    }

    pub fn clear(&mut self) {
        self.erased.clear()
    }

    // noinspection DuplicatedCode
    pub fn iter(self) -> Box<Iter<'data, C>> {
        let iter = self.erased.iter();
        let iter = iter.map(|it| {
            let entity = it.0;
            // SAFETY: pointer from the holdable iter is valid
            let component = unsafe { &*(it.1.get() as *const _) };
            (entity, component)
        });
        Box::new(iter)
    }

    pub fn iter_mut(self) -> Box<IterMut<'data, C>> {
        let iter = self.erased.iter_mut();
        let iter = iter.map(|it| {
            let entity = it.0;
            // SAFETY: pointer from the holdable iter is valid
            let component = unsafe { &mut *(it.1.get() as *mut _) };
            (entity, component)
        });
        Box::new(iter)
    }
}

impl<'data, C> From<&'data mut ErasedStorageHolder> for StorageHolderMut<'data, C>
where
    C: Component,
{
    fn from(erased: &'data mut ErasedStorageHolder) -> Self {
        debug_assert_eq!(erased.type_id(), &ComponentTypeId::of::<C>());
        Self {
            erased,
            _ph: PhantomData,
        }
    }
}
