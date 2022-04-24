use std::collections::HashMap;
use std::hash::BuildHasherDefault;

use as_any::Downcast;

use crate::world::TypeIdHasher;

use super::type_id::ResourceTypeId;
use super::Resource;

/// Storage of the resources - singletons in ECS.
///
/// Use this to [create][`ResourceStorage::create`] and [destroy][`ResourceStorage::destroy`]
/// resources, get resources [immutably][`ResourceStorage::get`]
/// or [mutably][`ResourceStorage::get_mut`].
#[derive(Default)]
#[repr(transparent)]
pub struct Registry {
    resources: HashMap<ResourceTypeId, Box<dyn Resource>, BuildHasherDefault<TypeIdHasher>>,
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
        self.resources.insert(type_id, Box::new(resource));
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
        let resource = resource.as_ref().downcast_ref().expect("downcast error");
        Some(resource)
    }

    pub fn get_mut<R>(&mut self) -> Option<&mut R>
    where
        R: Resource,
    {
        let type_id = ResourceTypeId::of::<R>();
        let resource = self.resources.get_mut(&type_id)?;
        let resource = resource.as_mut().downcast_mut().expect("downcast error");
        Some(resource)
    }
}
