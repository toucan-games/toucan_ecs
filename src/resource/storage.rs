use std::collections::HashMap;
use std::sync::Mutex;

use super::type_id::ResourceTypeId;
use super::{Ref, RefMut, Resource};

/// Storage of the resources - singletons in ECS.
///
/// Use this to [create][`ResourceStorage::create`] and [destroy][`ResourceStorage::destroy`]
/// resources, get resources [immutably][`ResourceStorage::get`]
/// or [mutably][`ResourceStorage::get_mut`].
#[derive(Default)]
pub struct ResourceStorage {
    resources: HashMap<ResourceTypeId, Mutex<Box<dyn Resource>>>,
}

impl ResourceStorage {
    pub fn new() -> Self {
        Self {
            resources: HashMap::new(),
        }
    }

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
        self.resources
            .insert(type_id, Mutex::new(Box::new(resource)));
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

    pub fn get<R>(&self) -> Option<Ref<R>>
    where
        R: Resource,
    {
        let type_id = ResourceTypeId::of::<R>();
        let resource = self.resources.get(&type_id)?;
        let resource = Ref::new(resource.lock().unwrap());
        Some(resource)
    }

    pub fn get_mut<R>(&self) -> Option<RefMut<R>>
    where
        R: Resource,
    {
        let type_id = ResourceTypeId::of::<R>();
        let resource = self.resources.get(&type_id)?;
        let resource = RefMut::new(resource.lock().unwrap());
        Some(resource)
    }
}
