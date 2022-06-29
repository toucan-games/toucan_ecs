use crate::component::{Component, ComponentTypeId, Storage};
use crate::Entity;

use super::holdable::{Holdable, Iter};

pub type ErasedComponent = usize;

pub struct ErasedStorageHolder {
    holdable: Box<dyn Holdable>,
    type_id: ComponentTypeId,
}

impl ErasedStorageHolder {
    /// # Safety
    ///
    /// Data behind provided erased component must be
    /// of type of storage which implements this trait and pointer must be valid.
    pub(super) unsafe fn attach(&mut self, entity: Entity, component: ErasedComponent) {
        self.holdable.attach(entity, component)
    }

    pub fn attached(&self, entity: Entity) -> bool {
        self.holdable.attached(entity)
    }

    /// # Safety
    ///
    /// Returned erased component points to the **immutable** data
    /// of type of storage which implements this trait.
    pub(super) fn get(&self, entity: Entity) -> Option<ErasedComponent> {
        self.holdable.get(entity)
    }

    /// # Safety
    ///
    /// Returned erased component points to the **mutable** data
    /// of type of storage which implements this trait.
    pub(super) fn get_mut(&mut self, entity: Entity) -> Option<ErasedComponent> {
        self.holdable.get_mut(entity)
    }

    pub fn remove(&mut self, entity: Entity) {
        self.holdable.remove(entity)
    }

    pub fn clear(&mut self) {
        self.holdable.clear()
    }

    pub(super) fn type_id(&self) -> &ComponentTypeId {
        &self.type_id
    }

    pub(super) fn iter(&self) -> Box<Iter> {
        self.holdable.iter()
    }

    pub(super) fn iter_mut(&mut self) -> Box<Iter> {
        self.holdable.iter_mut()
    }
}

impl<T, C> From<T> for ErasedStorageHolder
where
    T: Storage<Item = C>,
    C: Component,
{
    fn from(storage: T) -> Self {
        let holdable = Box::new(storage) as Box<_>;
        let type_id = ComponentTypeId::of::<C>();
        Self { holdable, type_id }
    }
}
