use std::collections::HashMap;

use crate::Resource;

use super::type_id::ResourceTypeId;

/// Storage of the resources - singletons in ECS.
///
/// Use this to [create][`ResourceStorage::create`] and [destroy][`ResourceStorage::destroy`]
/// resources, get resources [immutably][`ResourceStorage::get`]
/// or [mutably][`ResourceStorage::get_mut`].
#[derive(Default)]
pub struct ResourceStorage {
    resources: HashMap<ResourceTypeId, Box<dyn Resource>>,
}

impl ResourceStorage {
    /// Creates an empty storage with no resources.
    ///
    /// # Examples
    ///
    /// ```
    /// # use toucan_ecs::ResourceStorage;
    /// let resources = ResourceStorage::new();
    /// ```
    pub fn new() -> Self {
        Self {
            resources: HashMap::new(),
        }
    }

    /// Returns `true` if storage does not contain any resource.
    ///
    /// # Examples
    ///
    /// ```
    /// # use toucan_ecs::ResourceStorage;
    /// let mut resources = ResourceStorage::new();
    /// assert!(resources.is_empty());
    ///
    /// resources.create(42);
    /// assert!(!resources.is_empty());
    /// ```
    pub fn is_empty(&self) -> bool {
        self.resources.is_empty()
    }

    /// Clears this storage, destroying all resources.
    ///
    /// # Examples
    ///
    /// ```
    /// # use toucan_ecs::ResourceStorage;
    /// struct Resource(u32);
    ///
    /// let mut resources = ResourceStorage::new();
    ///
    /// resources.create(Resource(42));
    /// assert!(!resources.is_empty());
    ///
    /// resources.clear();
    /// assert!(resources.is_empty());
    /// ```
    pub fn clear(&mut self) {
        self.resources.clear();
    }

    /// Creates new resource and store it in the storage.
    ///
    /// To get created resource, call [`get`][ResourceStorage::get] or
    /// [`get_mut`][ResourceStorage::get_mut] associated function.
    ///
    /// # Examples
    ///
    /// ```
    /// # use toucan_ecs::ResourceStorage;
    /// struct Resource(u32);
    ///
    /// let mut resources = ResourceStorage::new();
    ///
    /// resources.create(Resource(42));
    /// assert!(!resources.is_empty());
    /// ```
    pub fn create<R>(&mut self, resource: R)
    where
        R: Resource,
    {
        let type_id = ResourceTypeId::of::<R>();
        self.resources.insert(type_id, Box::new(resource));
    }

    /// Destroys the resource of generic type and removes it from the storage.
    ///
    /// # Examples
    ///
    /// ```
    /// # use toucan_ecs::ResourceStorage;
    /// struct Resource(u32);
    ///
    /// let mut resources = ResourceStorage::new();
    ///
    /// resources.create(Resource(42));
    /// resources.destroy::<Resource>();
    /// assert!(!resources.contains::<Resource>());
    /// ```
    pub fn destroy<R>(&mut self)
    where
        R: Resource,
    {
        let type_id = ResourceTypeId::of::<R>();
        self.resources.remove(&type_id);
    }

    /// Returns `true` if storage has resource of generic type.
    ///
    /// # Examples
    ///
    /// ```
    /// # use toucan_ecs::ResourceStorage;
    /// struct Resource(u32);
    ///
    /// let mut resources = ResourceStorage::new();
    ///
    /// resources.create(Resource(42));
    /// assert!(resources.contains::<Resource>());
    /// ```
    pub fn contains<R>(&self) -> bool
    where
        R: Resource,
    {
        let type_id = ResourceTypeId::of::<R>();
        self.resources.contains_key(&type_id)
    }

    /// Retrieves the shared borrow of the generic resource type.
    ///
    /// # Examples
    ///
    /// ```
    /// # use toucan_ecs::ResourceStorage;
    /// #[derive(Debug, Eq, PartialEq)]
    /// struct Resource(u32);
    ///
    /// let mut resources = ResourceStorage::new();
    ///
    /// resources.create(Resource(42));
    /// let resource = resources.get::<Resource>().unwrap();
    /// assert_eq!(*resource, Resource(42));
    /// ```
    pub fn get<R>(&self) -> Option<&R>
    where
        R: Resource,
    {
        let type_id = ResourceTypeId::of::<R>();
        let resource = self.resources.get(&type_id)?;
        let resource = resource
            .as_ref()
            .as_any_ref()
            .downcast_ref()
            .expect("downcast error");
        Some(resource)
    }

    /// Retrieves the unique borrow of the generic resource type.
    ///
    /// # Examples
    ///
    /// ```
    /// # use toucan_ecs::ResourceStorage;
    /// #[derive(Debug, Eq, PartialEq)]
    /// struct Resource(u32);
    ///
    /// let mut resources = ResourceStorage::new();
    /// resources.create(Resource(42));
    ///
    /// let mut resource = resources.get_mut::<Resource>().unwrap();
    /// *resource = Resource(35);
    /// assert_eq!(*resource, Resource(35));
    /// ```
    pub fn get_mut<R>(&mut self) -> Option<&mut R>
    where
        R: Resource,
    {
        let type_id = ResourceTypeId::of::<R>();
        let resource = self.resources.get_mut(&type_id)?;
        let resource = resource
            .as_mut()
            .as_any_mut()
            .downcast_mut()
            .expect("downcast error");
        Some(resource)
    }
}
