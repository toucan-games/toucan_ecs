use crate::component::{Component, Storage};
use crate::Entity;

use super::erased::ErasedComponent;

pub type Iter<'data> = dyn Iterator<Item = (Entity, ErasedComponent)> + Send + Sync + 'data;

pub trait Holdable: Send + Sync + 'static {
    /// # Safety
    ///
    /// Data behind provided erased component must be
    /// of type of storage which implements this trait and pointer must be valid.
    unsafe fn attach(&mut self, entity: Entity, component: ErasedComponent);

    fn attached(&self, entity: Entity) -> bool;

    /// # Safety
    ///
    /// Returned erased component points to the **immutable** data
    /// of type of storage which implements this trait.
    fn get(&self, entity: Entity) -> Option<ErasedComponent>;

    /// # Safety
    ///
    /// Returned erased component points to the **mutable** data
    /// of type of storage which implements this trait.
    fn get_mut(&mut self, entity: Entity) -> Option<ErasedComponent>;

    fn remove(&mut self, entity: Entity);

    fn clear(&mut self);

    /// # Safety
    ///
    /// Returned iterator with erased components points to the **immutable** data
    /// of type of storage which implements this trait.
    // fixme move to associated type when GATs are stabilized
    fn iter(&self) -> Box<Iter>;

    /// # Safety
    ///
    /// Returned iterator with erased components points to the **mutable** data
    /// of type of storage which implements this trait.
    // fixme move to associated type when GATs are stabilized
    fn iter_mut(&mut self) -> Box<Iter>;
}

impl<T, C> Holdable for T
where
    T: Storage<Item = C>,
    C: Component,
{
    unsafe fn attach(&mut self, entity: Entity, component: ErasedComponent) {
        let component = *(component.get() as *const _);
        self.attach(entity, component)
    }

    fn attached(&self, entity: Entity) -> bool {
        self.attached(entity)
    }

    fn get(&self, entity: Entity) -> Option<ErasedComponent> {
        let component = self.get(entity)?;
        // SAFETY: component reference cannot be null
        let erased = unsafe { ErasedComponent::new_unchecked(component as *const _ as _) };
        Some(erased)
    }

    fn get_mut(&mut self, entity: Entity) -> Option<ErasedComponent> {
        let component = self.get_mut(entity)?;
        // SAFETY: component reference cannot be null
        let erased = unsafe { ErasedComponent::new_unchecked(component as *mut _ as _) };
        Some(erased)
    }

    fn remove(&mut self, entity: Entity) {
        self.remove(entity)
    }

    fn clear(&mut self) {
        self.clear()
    }

    fn iter(&self) -> Box<Iter> {
        let iter = self.iter();
        let iter = iter.map(|it| {
            let entity = it.0;
            // SAFETY: component reference cannot be null
            let erased = unsafe { ErasedComponent::new_unchecked(it.1 as *const _ as _) };
            (entity, erased)
        });
        Box::new(iter)
    }

    fn iter_mut(&mut self) -> Box<Iter> {
        let iter = self.iter_mut();
        let iter = iter.map(|it| {
            let entity = it.0;
            // SAFETY: component reference cannot be null
            let erased = unsafe { ErasedComponent::new_unchecked(it.1 as *mut _ as _) };
            (entity, erased)
        });
        Box::new(iter)
    }
}
