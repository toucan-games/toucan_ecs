use std::collections::HashMap;
use std::hash::BuildHasherDefault;

use atomicell::{AtomicCell, Ref, RefMut};

use crate::hash::TypeIdHasher;

use super::{ErasedResourceHolder, Resource, ResourceTypeId};

type ResourceRefCell = AtomicCell<ErasedResourceHolder>;

#[derive(Default)]
#[repr(transparent)]
pub struct Registry {
    resources: HashMap<ResourceTypeId, ResourceRefCell, BuildHasherDefault<TypeIdHasher>>,
}

impl Registry {
    pub fn is_empty(&self) -> bool {
        self.resources.is_empty()
    }

    pub fn clear(&mut self) {
        self.resources.clear();
    }

    pub fn create<R>(&mut self, resource: R)
    where
        R: Resource,
    {
        let type_id = ResourceTypeId::of::<R>();
        let holder = (resource,).into();
        self.resources.insert(type_id, AtomicCell::new(holder));
    }

    pub fn destroy<R>(&mut self)
    where
        R: Resource,
    {
        let type_id = ResourceTypeId::of::<R>();
        self.resources.remove(&type_id);
    }

    pub fn contains<R>(&self) -> bool
    where
        R: Resource,
    {
        let type_id = ResourceTypeId::of::<R>();
        self.resources.contains_key(&type_id)
    }

    pub fn get<R>(&self) -> Option<&R>
    where
        R: Resource,
    {
        let type_id = ResourceTypeId::of::<R>();
        let resource = self.resources.get(&type_id)?;
        // SAFETY: safe to use inside of the crate
        let resource = unsafe { resource.try_borrow_unguarded() }
            .expect("resource was already borrowed as mutable");
        let resource = resource.downcast_ref().expect("downcast error");
        Some(resource)
    }

    pub fn get_guarded<R>(&self) -> Option<Ref<R>>
    where
        R: Resource,
    {
        let type_id = ResourceTypeId::of::<R>();
        let resource = self.resources.get(&type_id)?;
        let resource = Ref::map(resource.borrow(), |erased| {
            erased.downcast_ref().expect("downcast error")
        });
        Some(resource)
    }

    pub fn get_mut<R>(&mut self) -> Option<&mut R>
    where
        R: Resource,
    {
        let type_id = ResourceTypeId::of::<R>();
        let resource = self.resources.get_mut(&type_id)?;
        let resource = resource.get_mut().downcast_mut().expect("downcast error");
        Some(resource)
    }

    pub fn get_mut_guarded<R>(&self) -> Option<RefMut<R>>
    where
        R: Resource,
    {
        let type_id = ResourceTypeId::of::<R>();
        let resource = self.resources.get(&type_id)?;
        let resource = RefMut::map(resource.borrow_mut(), |erased| {
            erased.downcast_mut().expect("downcast error")
        });
        Some(resource)
    }

    pub(crate) fn undo_leak(&mut self) {
        for resource in self.resources.values_mut() {
            resource.undo_leak();
        }
    }
}
