use crate::component::RegistryRefs as StorageRefs;
use crate::resource::{Registry as ResourceRegistry, RegistryRefs as ResourceRefs, Resource};
use crate::system::foreach::ForeachHolder;
use crate::world::query::{ResourceQuery, ResourceQueryMut};
use crate::world::WorldRefs;

/// **Immutable** borrowed type of the [world](crate::world::World) that contains
/// data of its resources.
#[cfg_attr(docsrs, doc(cfg(feature = "resource")))]
pub struct Resources<'world> {
    resources: &'world ResourceRegistry,
}

impl<'world> Resources<'world> {
    pub(super) fn new(resources: &'world ResourceRegistry) -> Self {
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

    /// Retrieves multiple **shared** resource borrows by provided query.
    ///
    /// # Panics
    ///
    /// Function will panic if requested resources cannot be retrieved.
    /// For example, if provided query contains resource which does not exist in the world,
    /// it will panic.
    ///
    /// # Examples
    ///
    /// ```
    /// # use toucan_ecs::prelude::*;
    /// use toucan_ecs::marker;
    ///
    /// #[derive(Debug, Eq, PartialEq, Resource)]
    /// struct ExampleResource(u32);
    ///
    /// #[derive(Debug, Eq, PartialEq, Resource)]
    /// struct AnotherResource;
    ///
    /// let mut world = World::new();
    /// world.create_resources(ExampleResource(42));
    ///
    /// type Query<'a> = (
    ///     marker::Resource<'a, ExampleResource>,
    ///     Option<marker::Resource<'a, AnotherResource>>,
    /// );
    /// let resources = world.resources();
    /// let (example, another) = resources.view::<Query>();
    ///
    /// assert_eq!(*example, ExampleResource(42));
    /// assert!(another.is_none());
    /// ```
    ///
    /// [rust_book]: https://doc.rust-lang.org/book/ch04-02-references-and-borrowing.html#the-rules-of-references
    pub fn view<'view, Q>(&'view self) -> Q
    where
        Q: ResourceQuery<'view>,
    {
        let mut data = WorldRefs {
            storages: StorageRefs::empty(),
            resources: ResourceRefs::from(self.resources),
        };
        ForeachHolder::new(None, &mut data)
            .next()
            .expect("unable to view resources by provided query")
    }
}

/// **Mutable** borrowed type of the [world](crate::world::World) that contains
/// data of its resources.
#[cfg_attr(docsrs, doc(cfg(feature = "resource")))]
pub struct ResourcesMut<'world> {
    resources: &'world mut ResourceRegistry,
}

impl<'world> ResourcesMut<'world> {
    pub(super) fn new(resources: &'world mut ResourceRegistry) -> Self {
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

    /// Retrieves multiple **shared** resource borrows by provided query.
    ///
    /// # Panics
    ///
    /// Function will panic if requested resources cannot be retrieved.
    /// For example, if provided query contains resource which does not exist in the world,
    /// it will panic.
    ///
    /// # Examples
    ///
    /// ```
    /// # use toucan_ecs::prelude::*;
    /// use toucan_ecs::marker;
    ///
    /// #[derive(Debug, Eq, PartialEq, Resource)]
    /// struct ExampleResource(u32);
    ///
    /// #[derive(Debug, Eq, PartialEq, Resource)]
    /// struct AnotherResource;
    ///
    /// let mut world = World::new();
    /// world.create_resources(ExampleResource(42));
    ///
    /// type Query<'a> = (
    ///     marker::Resource<'a, ExampleResource>,
    ///     Option<marker::Resource<'a, AnotherResource>>,
    /// );
    /// let resources = world.resources_mut();
    /// let (example, another) = resources.view::<Query>();
    ///
    /// assert_eq!(*example, ExampleResource(42));
    /// assert!(another.is_none());
    /// ```
    ///
    /// [rust_book]: https://doc.rust-lang.org/book/ch04-02-references-and-borrowing.html#the-rules-of-references
    pub fn view<'view, Q>(&'view self) -> Q
    where
        Q: ResourceQuery<'view>,
    {
        let mut data = WorldRefs {
            storages: StorageRefs::empty(),
            resources: ResourceRefs::from(&*self.resources),
        };
        ForeachHolder::new(None, &mut data)
            .next()
            .expect("unable to view resources by provided query")
    }

    /// Retrieves multiple **unique** resource borrows by provided query.
    ///
    /// # Panics
    ///
    /// Function will panic if requested resources cannot be retrieved.
    /// For example, if provided query contains resource which does not exist in the world,
    /// it will panic.
    ///
    /// Also this function will panic if provided query does not satisfies
    /// the first rule of references described in
    /// **References and Borrowing** section of [**Rust Book**][rust_book]:
    ///
    /// > - *At any given time, you can have either **one** mutable reference
    /// or **any** number of immutable references.*
    ///
    /// # Examples
    ///
    /// ```
    /// # use toucan_ecs::prelude::*;
    /// use toucan_ecs::marker;
    ///
    /// #[derive(Debug, Eq, PartialEq, Resource)]
    /// struct ExampleResource(u32);
    ///
    /// #[derive(Debug, Eq, PartialEq, Resource)]
    /// struct AnotherResource;
    ///
    /// let mut world = World::new();
    /// world.create_resources(ExampleResource(42));
    ///
    /// type Query<'a> = (
    ///     marker::ResourceMut<'a, ExampleResource>,
    ///     Option<marker::Resource<'a, AnotherResource>>,
    /// );
    /// let mut resources = world.resources_mut();
    /// let (mut example, another) = resources.view_mut::<Query>();
    ///
    /// example.0 = 10;
    /// assert_eq!(*example, ExampleResource(10));
    /// assert!(another.is_none());
    /// ```
    ///
    /// [rust_book]: https://doc.rust-lang.org/book/ch04-02-references-and-borrowing.html#the-rules-of-references
    pub fn view_mut<'view, Q>(&'view mut self) -> Q
    where
        Q: ResourceQueryMut<'view>,
    {
        let mut data = WorldRefs {
            storages: StorageRefs::empty(),
            resources: ResourceRefs::from(&mut *self.resources),
        };
        ForeachHolder::new(None, &mut data)
            .next()
            .expect("unable to view resources by provided query")
    }
}
