use crate::resource::{Registry as ResourceRegistry, Resource};

/// **Immutable** borrowed type of the [world](crate::world::World) that contains
/// data of its resources.
#[cfg_attr(docsrs, doc(cfg(feature = "resource")))]
pub struct Resources<'data> {
    resources: &'data ResourceRegistry,
}

impl<'data> Resources<'data> {
    pub(super) fn new(resources: &'data ResourceRegistry) -> Self {
        Self { resources }
    }

    /// Returns `true` if the world has resource of generic type.
    ///
    /// # Examples
    ///
    /// ```
    /// # use toucan_ecs::prelude::*;
    /// #[derive(Resource)]
    /// struct ExampleResource(u32);
    ///
    /// let mut world = World::new();
    /// world.create_resources(ExampleResource(42));
    ///
    /// let resources = world.resources();
    /// assert!(resources.contains::<ExampleResource>());
    /// ```
    pub fn contains<R>(&self) -> bool
    where
        R: Resource,
    {
        self.resources.contains::<R>()
    }

    /// Returns `true` if the world does not contain any resource.
    ///
    /// # Examples
    ///
    /// ```
    /// # use toucan_ecs::prelude::*;
    /// let mut world = World::new();
    /// let resources = world.resources();
    /// assert!(resources.is_empty());
    /// ```
    pub fn is_empty(&self) -> bool {
        self.resources.is_empty()
    }

    /// Retrieves the shared borrow of the generic resource type.
    ///
    /// # Examples
    ///
    /// ```
    /// # use toucan_ecs::prelude::*;
    /// #[derive(Debug, Eq, PartialEq, Resource)]
    /// struct ExampleResource(u32);
    ///
    /// let mut world = World::new();
    /// world.create_resources(ExampleResource(42));
    ///
    /// let resources = world.resources();
    /// let resource = resources.get::<ExampleResource>().unwrap();
    /// assert_eq!(*resource, ExampleResource(42));
    /// ```
    pub fn get<R>(&self) -> Option<&R>
    where
        R: Resource,
    {
        self.resources.get::<R>()
    }
}

/// **Mutable** borrowed type of the [world](crate::world::World) that contains
/// data of its resources.
#[cfg_attr(docsrs, doc(cfg(feature = "resource")))]
pub struct ResourcesMut<'data> {
    resources: &'data mut ResourceRegistry,
}

impl<'data> ResourcesMut<'data> {
    pub(super) fn new(resources: &'data mut ResourceRegistry) -> Self {
        Self { resources }
    }

    /// Returns `true` if the world has resource of generic type.
    ///
    /// # Examples
    ///
    /// ```
    /// # use toucan_ecs::prelude::*;
    /// #[derive(Resource)]
    /// struct ExampleResource(u32);
    ///
    /// let mut world = World::new();
    /// world.create_resources(ExampleResource(42));
    ///
    /// let resources = world.resources_mut();
    /// assert!(resources.contains::<ExampleResource>());
    /// ```
    pub fn contains<R>(&self) -> bool
    where
        R: Resource,
    {
        self.resources.contains::<R>()
    }

    /// Returns `true` if the world does not contain any resource.
    ///
    /// # Examples
    ///
    /// ```
    /// # use toucan_ecs::prelude::*;
    /// let mut world = World::new();
    /// let resources = world.resources();
    /// assert!(resources.is_empty());
    /// ```
    pub fn is_empty(&self) -> bool {
        self.resources.is_empty()
    }

    /// Retrieves the shared borrow of the generic resource type.
    ///
    /// # Examples
    ///
    /// ```
    /// # use toucan_ecs::prelude::*;
    /// #[derive(Debug, Eq, PartialEq, Resource)]
    /// struct ExampleResource(u32);
    ///
    /// let mut world = World::new();
    /// world.create_resources(ExampleResource(42));
    ///
    /// let resources = world.resources_mut();
    /// let resource = resources.get::<ExampleResource>().unwrap();
    /// assert_eq!(*resource, ExampleResource(42));
    /// ```
    pub fn get<R>(&self) -> Option<&R>
    where
        R: Resource,
    {
        self.resources.get::<R>()
    }

    /// Retrieves the unique borrow of the generic resource type.
    ///
    /// # Examples
    ///
    /// ```
    /// # use toucan_ecs::prelude::*;
    /// #[derive(Debug, Eq, PartialEq, Resource)]
    /// struct ExampleResource(u32);
    ///
    /// let mut world = World::new();
    /// world.create_resources(ExampleResource(42));
    ///
    /// let mut resources = world.resources_mut();
    /// let mut resource = resources.get_mut::<ExampleResource>().unwrap();
    /// *resource = ExampleResource(35);
    /// assert_eq!(*resource, ExampleResource(35));
    /// ```
    pub fn get_mut<R>(&mut self) -> Option<&mut R>
    where
        R: Resource,
    {
        self.resources.get_mut::<R>()
    }
}
