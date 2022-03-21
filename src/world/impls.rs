use crate::component::{Component, ComponentSet, Entry, Registry};
use crate::entity::Entity;
#[cfg(feature = "resource")]
use crate::resource::{Registry as ResourceRegistry, Resource};

/// Storage of the entities and all the data attached to them.
/// Additionally can store resources if enabled by the feature `resource`.
///
/// Use this to [create][`World::create`] and [destroy][`World::destroy`] entities,
/// [attach][`World::attach`] and [remove][`World::remove`] components' data of the entity,
/// [create][`World::entry`] entry for the entity,
/// view each component separately or group of components together.
#[derive(Default)]
pub struct World {
    registry: Registry,
    #[cfg(feature = "resource")]
    resources: ResourceRegistry,
}

impl World {
    /// Creates an empty world with no entities and no data.
    ///
    /// # Examples
    ///
    /// ```
    /// # use toucan_ecs::World;
    /// let world = World::new();
    /// ```
    pub fn new() -> Self {
        Self {
            registry: Registry::new(),
            #[cfg(feature = "resource")]
            resources: ResourceRegistry::new(),
        }
    }

    /// Creates new entity with no data attached to it.
    ///
    /// To attach some data to the entity, use [`attach`][`World::attach`]
    /// or [`attach_one`][`World::attach_one`] associated functions.
    ///
    /// # Examples
    ///
    /// ```
    /// # use toucan_ecs::World;
    /// let mut world = World::new();
    ///
    /// let entity = world.create();
    /// assert!(world.contains(entity));
    /// assert!(world.is_entity_empty(entity));
    /// ```
    pub fn create(&mut self) -> Entity {
        self.registry.create()
    }

    /// Creates new resource and stores it in the world.
    ///
    /// To get created resource, call [`get_resource`][World::get_resource] or
    /// [`get_resource_mut`][World::get_resource_mut] associated function.
    ///
    /// # Examples
    ///
    /// ```
    /// # use toucan_ecs::World;
    /// struct Resource(u32);
    ///
    /// let mut world = World::new();
    ///
    /// world.create_resource(Resource(42));
    /// assert!(!world.is_empty());
    /// ```
    #[cfg(feature = "resource")]
    pub fn create_resource<R>(&mut self, resource: R)
    where
        R: Resource,
    {
        self.resources.create(resource)
    }

    /// Creates new entity with one component attached to it.
    ///
    /// This can be done by hand with [`attach_one`][`World::attach_one`] associated function.
    ///
    /// # Examples
    ///
    /// ```
    /// # use toucan_ecs::World;
    /// #[derive(Copy, Clone)]
    /// struct Name(&'static str);
    ///
    /// let mut world = World::new();
    /// let entity = world.create_with_one(Name("Hello, World"));
    /// assert!(world.contains(entity));
    /// assert!(!world.is_entity_empty(entity));
    /// ```
    pub fn create_with_one<C>(&mut self, component: C) -> Entity
    where
        C: Component,
    {
        self.registry.create_with_one(component)
    }

    /// Creates new entity with set of components attached to it.
    ///
    /// This can be done by hand with [`attach`][`World::attach`] associated function.
    ///
    /// # Examples
    ///
    /// ```
    /// # use toucan_ecs::World;
    /// #[derive(Copy, Clone)]
    /// struct Name(&'static str);
    ///
    /// #[derive(Copy, Clone)]
    /// struct ID(u32);
    ///
    /// let mut world = World::new();
    /// let entity = world.create_with((Name("Hello, World"), ID(42)));
    /// assert!(world.contains(entity));
    /// assert!(!world.is_entity_empty(entity));
    /// ```
    pub fn create_with<S>(&mut self, set: S) -> Entity
    where
        S: ComponentSet,
    {
        self.registry.create_with(set)
    }

    /// Creates new [entry][`Entry`] for the newly created entity.
    ///
    /// # Examples
    ///
    /// ```
    /// # use toucan_ecs::World;
    /// let mut world = World::new();
    ///
    /// let entry = world.create_entry();
    /// let entity = entry.entity();
    /// assert!(world.contains(entity));
    /// assert!(world.is_entity_empty(entity));
    /// ```
    pub fn create_entry(&mut self) -> Entry {
        self.registry.create_entry()
    }

    /// Creates [entry][`Entry`] for the newly created entity with one component attached to it.
    ///
    /// This can be done by hand with [`Entry::attach_one`] associated function.
    ///
    /// # Examples
    ///
    /// ```
    /// # use toucan_ecs::World;
    /// #[derive(Copy, Clone)]
    /// struct Name(&'static str);
    ///
    /// let mut world = World::new();
    /// let entry = world.create_entry_with_one(Name("Hello, World"));
    ///
    /// let entity = entry.entity();
    /// assert!(world.contains(entity));
    /// assert!(!world.is_entity_empty(entity));
    /// ```
    pub fn create_entry_with_one<C>(&mut self, component: C) -> Entry
    where
        C: Component,
    {
        self.registry.create_entry_with_one(component)
    }

    /// Creates [entry][`Entry`] for the newly created entity with set of components attached to it.
    ///
    /// This can be done by hand with [`Entry::attach`] associated function.
    ///
    /// # Examples
    ///
    /// ```
    /// # use toucan_ecs::World;
    /// #[derive(Copy, Clone)]
    /// struct Name(&'static str);
    ///
    /// #[derive(Copy, Clone)]
    /// struct ID(u32);
    ///
    /// let mut world = World::new();
    /// let entry = world.create_entry_with((Name("Hello, World"), ID(42)));
    ///
    /// let entity = entry.entity();
    /// assert!(world.contains(entity));
    /// assert!(!world.is_entity_empty(entity));
    /// ```
    pub fn create_entry_with<S>(&mut self, set: S) -> Entry
    where
        S: ComponentSet,
    {
        self.registry.create_entry_with(set)
    }

    /// Create [entry][`Entry`] for the provided entity.
    ///
    /// Returns [`None`][`Option::None`] if the provided entity was previously destroyed.
    ///
    /// # Examples
    ///
    /// ```
    /// # use toucan_ecs::World;
    /// let mut world = World::new();
    ///
    /// let entity = world.create();
    /// assert!(world.entry(entity).is_some());
    ///
    /// world.destroy(entity);
    /// assert!(world.entry(entity).is_none());
    /// ```
    pub fn entry(&mut self, entity: Entity) -> Option<Entry> {
        self.registry.entry(entity)
    }

    /// Extends world with provided count of newly created entities.
    /// Returns handles of newly created entities.
    ///
    /// # Examples
    ///
    /// ```
    /// # use toucan_ecs::World;
    /// let mut world = World::new();
    ///
    /// let entities = world.extend(10);
    /// assert_eq!(entities.len(), 10);
    /// ```
    pub fn extend(&mut self, count: u32) -> &[Entity] {
        self.registry.extend(count)
    }

    /// Extends world with collection of components to the newly created entities.
    /// Returns handles of newly created entities.
    ///
    /// # Examples
    ///
    /// ```
    /// # use toucan_ecs::World;
    /// #[derive(Copy, Clone)]
    /// struct ID(u32);
    ///
    /// let mut world = World::new();
    ///
    /// let entities = world.extend_with_one([ID(1), ID(2), ID(3), ID(4), ID(5)]);
    /// assert_eq!(entities.len(), 5);
    /// ```
    pub fn extend_with_one<I, C>(&mut self, into_iter: I) -> &[Entity]
    where
        I: IntoIterator<Item = C>,
        C: Component,
    {
        self.registry.extend_with_one(into_iter)
    }

    // noinspection SpellCheckingInspection
    /// Extends world with collection of set of components to the newly created entities.
    /// Returns handles of newly created entities.
    ///
    /// # Examples
    ///
    /// ```
    /// # use toucan_ecs::World;
    /// #[derive(Copy, Clone)]
    /// struct Name(&'static str);
    ///
    /// #[derive(Copy, Clone)]
    /// struct ID(u32);
    ///
    /// let mut world = World::new();
    ///
    /// let entities = world.extend_with([
    ///     (Name("Hello, World"), ID(1)),
    ///     (Name("Привет, Мир"), ID(2)),
    ///     (Name("你好世界"), ID(3)),
    ///     (Name("नमस्ते दुनिया"), ID(4)),
    /// ]);
    /// assert_eq!(entities.len(), 4);
    /// ```
    pub fn extend_with<I, S>(&mut self, into_iter: I) -> &[Entity]
    where
        I: IntoIterator<Item = S>,
        S: ComponentSet,
    {
        self.registry.extend_with(into_iter)
    }

    /// Returns `true` if the world contains the entity.
    ///
    /// # Examples
    ///
    /// ```
    /// # use toucan_ecs::World;
    /// let mut world = World::new();
    ///
    /// let entity = world.create();
    /// assert!(world.contains(entity));
    ///
    /// world.destroy(entity);
    /// assert!(!world.contains(entity));
    /// ```
    pub fn contains(&self, entity: Entity) -> bool {
        self.registry.contains(entity)
    }

    /// Returns `true` if the world has resource of generic type.
    ///
    /// # Examples
    ///
    /// ```
    /// # use toucan_ecs::World;
    /// struct Resource(u32);
    ///
    /// let mut world = World::new();
    ///
    /// world.create_resource(Resource(42));
    /// assert!(world.contains_resource::<Resource>());
    /// ```
    #[cfg(feature = "resource")]
    pub fn contains_resource<R>(&self) -> bool
    where
        R: Resource,
    {
        self.resources.contains::<R>()
    }

    /// Destroys the entity and removes all its attached components.
    ///
    /// # Examples
    ///
    /// ```
    /// # use toucan_ecs::World;
    /// let mut world = World::new();
    ///
    /// let entity = world.create();
    /// world.destroy(entity);
    /// assert!(!world.contains(entity));
    /// ```
    pub fn destroy(&mut self, entity: Entity) {
        self.registry.destroy(entity)
    }

    /// Destroys the resource of generic type and removes it from the world.
    ///
    /// # Examples
    ///
    /// ```
    /// # use toucan_ecs::World;
    /// struct Resource(u32);
    ///
    /// let mut world = World::new();
    ///
    /// world.create_resource(Resource(42));
    /// world.destroy_resource::<Resource>();
    /// assert!(!world.contains_resource::<Resource>());
    /// ```
    #[cfg(feature = "resource")]
    pub fn destroy_resource<R>(&mut self)
    where
        R: Resource,
    {
        self.resources.destroy::<R>();
    }

    /// Returns `true` if the world does not contain any entity and any resource.
    ///
    /// # Examples
    ///
    /// ```
    /// # use toucan_ecs::World;
    /// let mut world = World::new();
    /// assert!(world.is_empty());
    ///
    /// let _ = world.create();
    /// assert!(!world.is_empty());
    /// ```
    pub fn is_empty(&self) -> bool {
        self.cfg_is_empty()
    }

    #[cfg(feature = "resource")]
    #[inline(always)]
    fn cfg_is_empty(&self) -> bool {
        self.registry.is_empty() && self.resources.is_empty()
    }

    #[cfg(not(feature = "resource"))]
    #[inline(always)]
    fn cfg_is_empty(&self) -> bool {
        self.registry.is_empty()
    }

    /// Clears this world, destroying all resources, all entities and their data.
    ///
    /// # Examples
    ///
    /// ```
    /// # use toucan_ecs::World;
    /// let mut world = World::new();
    ///
    /// world.extend(100);
    /// assert!(!world.is_empty());
    ///
    /// world.clear();
    /// assert!(world.is_empty());
    /// ```
    pub fn clear(&mut self) {
        self.registry.clear();
        #[cfg(feature = "resource")]
        self.resources.clear();
    }

    /// Registers new type of component to be stored in the world.
    ///
    /// # Examples
    ///
    /// ```
    /// # use toucan_ecs::World;
    /// #[derive(Copy, Clone)]
    /// struct ID(u32);
    ///
    /// let mut world = World::new();
    /// world.register::<&'static str>();
    /// world.register::<ID>();
    /// ```
    pub fn register<C>(&mut self)
    where
        C: Component,
    {
        self.registry.register::<C>();
    }

    /// Attaches exactly one component to the entity.
    ///
    /// To attach multiple components of different types to the entity at once,
    /// use [`attach`][`World::attach`] associated function.
    ///
    /// # Examples
    ///
    /// ```
    /// # use toucan_ecs::World;
    /// #[derive(Copy, Clone, Eq, PartialEq, Debug)]
    /// struct Name(&'static str);
    ///
    /// let mut world = World::new();
    ///
    /// let entity = world.create();
    /// world.attach_one(entity, Name("Hello, World"));
    /// assert_eq!(world.get(entity).as_deref(), Some(&Name("Hello, World")));
    /// ```
    pub fn attach_one<C>(&mut self, entity: Entity, component: C)
    where
        C: Component,
    {
        self.registry.attach_one(entity, component)
    }

    /// Attaches set of components to the entity.
    ///
    /// To attach component of exactly one type to the entity,
    /// use [`attach_one`][`World::attach_one`] associated function.
    ///
    /// # Examples
    ///
    /// ```
    /// # use toucan_ecs::World;
    /// #[derive(Copy, Clone)]
    /// struct Name(&'static str);
    ///
    /// #[derive(Copy, Clone)]
    /// struct ID(u32);
    ///
    /// let mut world = World::new();
    ///
    /// let entity = world.create();
    /// world.attach(entity, (Name("Hello, World"), ID(42)));
    /// assert!(world.attached::<(ID, Name)>(entity));
    /// ```
    pub fn attach<S>(&mut self, entity: Entity, set: S)
    where
        S: ComponentSet,
    {
        self.registry.attach(entity, set)
    }

    /// Returns `true` if component of generic type is attached to the entity.
    ///
    /// To check if the entity has components of multiple types,
    /// use [`attached`][`World::attached`] associated function.
    ///
    /// # Examples
    ///
    /// ```
    /// # use toucan_ecs::World;
    /// #[derive(Copy, Clone)]
    /// struct Name(&'static str);
    ///
    /// let mut world = World::new();
    ///
    /// let entity = world.create();
    /// assert!(!world.attached_one::<Name>(entity));
    ///
    /// world.attach_one(entity, Name("Hello, World"));
    /// assert!(world.attached_one::<Name>(entity));
    /// ```
    pub fn attached_one<C>(&self, entity: Entity) -> bool
    where
        C: Component,
    {
        self.registry.attached_one::<C>(entity)
    }

    /// Returns `true` if components in the generic set type are attached to the entity.
    ///
    /// To check if the entity has component of exactly one type,
    /// use [`attached_one`][`World::attached_one`] associated function.
    ///
    /// # Examples
    ///
    /// ```
    /// # use toucan_ecs::World;
    /// #[derive(Copy, Clone)]
    /// struct Name(&'static str);
    ///
    /// #[derive(Copy, Clone)]
    /// struct ID(u32);
    ///
    /// let mut world = World::new();
    ///
    /// let entity = world.create();
    /// assert!(!world.attached::<(ID, Name)>(entity));
    ///
    /// world.attach(entity, (Name("Hello, World"), ID(42)));
    /// assert!(world.attached::<(Name, ID)>(entity));
    /// ```
    pub fn attached<S>(&self, entity: Entity) -> bool
    where
        S: ComponentSet,
    {
        self.registry.attached::<S>(entity)
    }

    /// Returns `true` if the entity does not exist or does not contain any data attached to it.
    ///
    /// # Examples
    ///
    /// ```
    /// # use toucan_ecs::World;
    /// let mut world = World::new();
    ///
    /// let entity = world.create();
    /// assert!(world.is_entity_empty(entity));
    /// ```
    pub fn is_entity_empty(&self, entity: Entity) -> bool {
        self.registry.is_entity_empty(entity)
    }

    /// Removes component of one type from the entity.
    ///
    /// To remove components of multiple types from the entity at once,
    /// use [`remove`][`World::remove`] associated function.
    ///
    /// # Examples
    ///
    /// ```
    /// # use toucan_ecs::World;
    /// #[derive(Copy, Clone)]
    /// struct Name(&'static str);
    ///
    /// let mut world = World::new();
    ///
    /// let entity = world.create_with((Name("Hello, World"),));
    /// assert!(world.attached_one::<Name>(entity));
    ///
    /// world.remove_one::<Name>(entity);
    /// assert!(!world.attached_one::<Name>(entity));
    /// ```
    pub fn remove_one<C>(&mut self, entity: Entity)
    where
        C: Component,
    {
        self.registry.remove_one::<C>(entity);
    }

    /// Removes components of multiple types from the entity.
    ///
    /// To remove component of one type from the entity,
    /// use [`remove_one`][`World::remove_one`] associated function.
    ///
    /// # Examples
    ///
    /// ```
    /// # use toucan_ecs::World;
    /// #[derive(Copy, Clone)]
    /// struct Name(&'static str);
    ///
    /// #[derive(Copy, Clone)]
    /// struct ID(u32);
    ///
    /// let mut world = World::new();
    ///
    /// let entity = world.create_with((Name("Hello, World"), ID(42)));
    /// world.remove::<(ID, Name)>(entity);
    /// assert!(!world.attached::<(Name, ID)>(entity));
    /// ```
    pub fn remove<S>(&mut self, entity: Entity)
    where
        S: ComponentSet,
    {
        self.registry.remove::<S>(entity);
    }

    /// Removes all attached components from the entity.
    /// It makes the entity effectively empty.
    ///
    /// To remove just a set of components from the entity,
    /// use [`remove_one`][`World::remove_one`] and [`remove`][`World::remove`]
    /// associated functions.
    ///
    /// # Examples
    ///
    /// ```
    /// # use toucan_ecs::World;
    /// #[derive(Copy, Clone)]
    /// struct Name(&'static str);
    ///
    /// #[derive(Copy, Clone)]
    /// struct ID(u32);
    ///
    /// let mut world = World::new();
    ///
    /// let entity = world.create_with((Name("Hello, World"), ID(42)));
    /// world.remove_all(entity);
    /// assert!(world.is_entity_empty(entity));
    /// ```
    pub fn remove_all(&mut self, entity: Entity) {
        self.registry.remove_all(entity);
    }

    /// Retrieves the shared borrow for the component of one type attached to the entity.
    /// Returns [`None`][`Option::None`] if component is not attached to the entity.
    ///
    /// # Examples
    ///
    /// ```
    /// # use toucan_ecs::World;
    /// #[derive(Copy, Clone, Eq, PartialEq, Debug)]
    /// struct Name(&'static str);
    ///
    /// let mut world = World::new();
    ///
    /// let entity = world.create_with((Name("Hello, World"),));
    /// let name = world.get::<Name>(entity).unwrap();
    /// assert_eq!(*name, Name("Hello, World"));
    /// ```
    pub fn get<C>(&self, entity: Entity) -> Option<&C>
    where
        C: Component,
    {
        self.registry.get::<C>(entity)
    }

    /// Retrieves the unique borrow for the component of one type attached to the entity.
    /// Returns [`None`][`Option::None`] if component is not attached to the entity.
    ///
    /// # Examples
    ///
    /// ```
    /// # use toucan_ecs::World;
    /// #[derive(Copy, Clone, Eq, PartialEq, Debug)]
    /// struct Name(&'static str);
    ///
    /// let mut world = World::new();
    ///
    /// let entity = world.create_with((Name("Hello, World"),));
    /// let mut name = world.get_mut::<Name>(entity).unwrap();
    /// name.0 = "This name was changed";
    /// assert_ne!(*name, Name("Hello, World"));
    /// assert_eq!(*name, Name("This name was changed"));
    /// ```
    pub fn get_mut<C>(&mut self, entity: Entity) -> Option<&mut C>
    where
        C: Component,
    {
        self.registry.get_mut::<C>(entity)
    }

    /// Retrieves the shared borrow of the generic resource type.
    ///
    /// # Examples
    ///
    /// ```
    /// # use toucan_ecs::World;
    /// #[derive(Debug, Eq, PartialEq)]
    /// struct Resource(u32);
    ///
    /// let mut world = World::new();
    ///
    /// world.create_resource(Resource(42));
    /// let resource = world.get_resource::<Resource>().unwrap();
    /// assert_eq!(*resource, Resource(42));
    /// ```
    #[cfg(feature = "resource")]
    pub fn get_resource<R>(&self) -> Option<&R>
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
    /// # use toucan_ecs::World;
    /// #[derive(Debug, Eq, PartialEq)]
    /// struct Resource(u32);
    ///
    /// let mut world = World::new();
    /// world.create_resource(Resource(42));
    ///
    /// let mut resource = world.get_resource_mut::<Resource>().unwrap();
    /// *resource = Resource(35);
    /// assert_eq!(*resource, Resource(35));
    /// ```
    #[cfg(feature = "resource")]
    pub fn get_resource_mut<R>(&mut self) -> Option<&mut R>
    where
        R: Resource,
    {
        self.resources.get_mut::<R>()
    }
}
