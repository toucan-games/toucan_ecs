use std::collections::HashMap;
use std::hash::BuildHasherDefault;

use crate::world::TypeIdHasher;

use super::{ErasedResourceHolder, Resource, ResourceHolder, ResourceTypeId};

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

    pub fn create<R>(&mut self, resource: R)
    where
        R: Resource,
    {
        let type_id = ResourceTypeId::of::<R>();
        self.resources.insert(type_id, (resource, ).into());
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
        let resource = resource.downcast_ref().expect("downcast error");
        Some(resource)
    }

    pub fn get_mut<R>(&mut self) -> Option<&mut R>
    where
        R: Resource,
    {
        let type_id = ResourceTypeId::of::<R>();
        let resource = self.resources.get_mut(&type_id)?;
        let resource = resource.downcast_mut().expect("downcast error");
        Some(resource)
    }

    pub fn get_holder<R>(&mut self) -> Option<ResourceHolder<R>>
    where
        R: Resource,
    {
        let type_id = ResourceTypeId::of::<R>();
        let resource = self.resources.remove(&type_id)?;
        Some(resource.into())
    }

    pub fn put_holder<R>(&mut self, resource_holder: ResourceHolder<R>)
    where
        R: Resource,
    {
        let type_id = ResourceTypeId::of::<R>();
        self.resources.insert(type_id, resource_holder.into());
    }
}
