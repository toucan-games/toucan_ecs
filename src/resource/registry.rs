use std::collections::HashMap;
use std::hash::BuildHasherDefault;

use crate::hash::TypeIdHasher;
use crate::resource::{ErasedResourceHolder, Resource, ResourceSet, ResourceTypeId};

#[derive(Default)]
#[repr(transparent)]
pub struct Registry {
    resources: HashMap<ResourceTypeId, ErasedResourceHolder, BuildHasherDefault<TypeIdHasher>>,
}

impl Registry {
    pub fn is_empty(&self) -> bool {
        self.resources.is_empty()
    }

    pub fn clear(&mut self) {
        self.resources.clear();
    }

    pub fn create<S>(&mut self, set: S)
    where
        S: ResourceSet,
    {
        set.create(self)
    }

    pub(super) fn create_one<R>(&mut self, resource: R)
    where
        R: Resource,
    {
        let type_id = ResourceTypeId::of::<R>();
        let erased = (resource,).into();
        self.resources.insert(type_id, erased);
    }

    pub fn destroy<S>(&mut self)
    where
        S: ResourceSet,
    {
        S::destroy(self)
    }

    pub(super) fn destroy_one<R>(&mut self)
    where
        R: Resource,
    {
        let type_id = ResourceTypeId::of::<R>();
        self.resources.remove(&type_id);
    }

    pub fn contains<S>(&self) -> bool
    where
        S: ResourceSet,
    {
        S::contains(self)
    }

    pub(super) fn contains_one<R>(&self) -> bool
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
        let resource = resource.as_resource_ref().expect("downcast error");
        Some(resource)
    }

    pub fn get_mut<R>(&mut self) -> Option<&mut R>
    where
        R: Resource,
    {
        let type_id = ResourceTypeId::of::<R>();
        let resource = self.resources.get_mut(&type_id)?;
        let resource = resource.as_resource_mut().expect("downcast error");
        Some(resource)
    }

    pub(super) fn iter(
        &self,
    ) -> impl Iterator<Item = (&ResourceTypeId, &ErasedResourceHolder)> + '_ {
        self.resources.iter()
    }

    pub(super) fn iter_mut(
        &mut self,
    ) -> impl Iterator<Item = (&ResourceTypeId, &mut ErasedResourceHolder)> + '_ {
        self.resources.iter_mut()
    }
}
