use crate::component::{
    Component, ComponentSet, Registry as ComponentRegistry, RegistryRefs as StorageRefs,
};
use crate::entity::{Entity, EntityBuilder, Registry as EntityRegistry};
#[cfg(feature = "resource")]
use crate::resource::{Registry as ResourceRegistry, RegistryRefs as ResourceRefs, Resource};
use crate::world::world_refs::WorldRefs;
use crate::world::Entry;

use super::query::{Query, QueryMut};
use super::view::{View, ViewMut, ViewOne, ViewOneMut};

/// Storage of the entities and all the data attached to them.
/// Additionally can store resources if enabled by the feature `resource`.
///
/// Use this to [create][World::create()] and [destroy][World::destroy()] entities,
/// [attach][World::attach()] and [remove][World::remove()] components' data of the entity,
/// [create entry][World::entry()] for the entity,
/// view each component separately or group of components together.
pub struct World {
    entities: EntityRegistry,
    components: ComponentRegistry,
    #[cfg(feature = "resource")]
    resources: ResourceRegistry,
}

impl Default for World {
    fn default() -> Self {
        Self::new()
    }
}

impl World {
    /// Creates an empty world with no entities and no data.
    ///
    /// # Examples
    ///
    /// ```
    /// # use toucan_ecs::prelude::*;
    /// let world = World::new();
    /// ```
    pub fn new() -> Self {
        Self {
            entities: EntityRegistry::default(),
            components: ComponentRegistry::default(),
            #[cfg(feature = "resource")]
            resources: ResourceRegistry::default(),
        }
    }

    /// Creates new entity with no data attached to it.
    ///
    /// To attach some data to the entity, use [`attach`][World::attach()] associated functions.
    ///
    /// # Examples
    ///
    /// ```
    /// # use toucan_ecs::prelude::*;
    /// let mut world = World::new();
    ///
    /// let entity = world.create();
    /// assert!(world.contains(entity));
    /// assert!(world.is_entity_empty(entity));
    /// ```
    pub fn create(&mut self) -> Entity {
        self.entities.create()
    }

    /// Creates new resource and stores it in the world.
    ///
    /// To get created resource, call [`get_resource`][World::get_resource()] or
    /// [`get_resource_mut`][World::get_resource_mut()] associated function.
    ///
    /// # Examples
    ///
    /// ```
    /// # use toucan_ecs::prelude::*;
    /// struct ExampleResource(u32);
    ///
    /// let mut world = World::new();
    ///
    /// world.create_resource(ExampleResource(42));
    /// assert!(!world.is_empty());
    /// ```
    #[cfg(feature = "resource")]
    #[cfg_attr(docsrs, doc(cfg(feature = "resource")))]
    pub fn create_resource<R>(&mut self, resource: R)
    where
        R: Resource,
    {
        self.resources.create(resource)
    }

    /// Creates new entity with one component or set of components attached to it.
    ///
    /// This can be done by hand with [`attach`][World::attach()] associated function.
    ///
    /// # Examples
    ///
    /// ```
    /// # use toucan_ecs::prelude::*;
    /// #[derive(Copy, Clone, Component)]
    /// struct Name(&'static str);
    ///
    /// #[derive(Copy, Clone, Component)]
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
        let entity = self.create();
        self.attach(entity, set);
        entity
    }

    /// Creates new [entry](Entry) for the newly created entity.
    ///
    /// # Examples
    ///
    /// ```
    /// # use toucan_ecs::prelude::*;
    /// let mut world = World::new();
    ///
    /// let entry = world.create_entry();
    /// let entity = entry.entity();
    /// assert!(world.contains(entity));
    /// assert!(world.is_entity_empty(entity));
    /// ```
    pub fn create_entry(&mut self) -> Entry {
        let entity = self.create();
        Entry::new(entity, self)
    }

    /// Creates [entry](Entry) for the newly created entity
    /// with one component or set of components attached to it.
    ///
    /// This can be done by hand with [`Entry::attach`] associated function.
    ///
    /// # Examples
    ///
    /// ```
    /// # use toucan_ecs::prelude::*;
    /// #[derive(Copy, Clone, Component)]
    /// struct Name(&'static str);
    ///
    /// #[derive(Copy, Clone, Component)]
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
        let entity = self.create_with(set);
        Entry::new(entity, self)
    }

    /// Creates [entry](Entry) for the provided entity.
    ///
    /// Returns [`None`](Option::None) if the provided entity was previously destroyed.
    ///
    /// # Examples
    ///
    /// ```
    /// # use toucan_ecs::prelude::*;
    /// let mut world = World::new();
    ///
    /// let entity = world.create();
    /// assert!(world.entry(entity).is_some());
    ///
    /// world.destroy(entity);
    /// assert!(world.entry(entity).is_none());
    /// ```
    pub fn entry(&mut self, entity: Entity) -> Option<Entry> {
        self.contains(entity).then(|| Entry::new(entity, self))
    }

    /// Creates new entity lazy builder which allows
    /// to attach components to new entity later.
    ///
    /// # Examples
    ///
    /// ```
    /// # use toucan_ecs::prelude::*;
    /// #[derive(Copy, Clone, Component)]
    /// struct Position {
    ///     x: f32,
    ///     y: f32,
    /// }
    ///
    /// #[derive(Copy, Clone, Component)]
    /// struct Mass(f32);
    ///
    /// let mut world = World::new();
    ///
    /// let entity = world.entity()
    ///     .with(Mass(10.0))
    ///     .with(Position { x: 100.0, y: -100.0 })
    ///     .build();
    ///
    /// assert!(world.contains(entity));
    /// assert!(world.attached::<(Position, Mass)>(entity));
    /// ```
    pub fn entity(&mut self) -> EntityBuilder {
        EntityBuilder::new(self)
    }

    /// Extends world with provided count of newly created entities.
    /// Returns handles of newly created entities.
    ///
    /// # Examples
    ///
    /// ```
    /// # use toucan_ecs::prelude::*;
    /// let mut world = World::new();
    ///
    /// let entities = world.extend(10);
    /// assert_eq!(entities.len(), 10);
    /// ```
    pub fn extend(&mut self, count: u32) -> Vec<Entity> {
        (0..count).map(|_| self.create()).collect()
    }

    // noinspection SpellCheckingInspection
    /// Extends world with collection of set of components to the newly created entities.
    /// Returns handles of newly created entities.
    ///
    /// # Examples
    ///
    /// ```
    /// # use toucan_ecs::prelude::*;
    /// #[derive(Copy, Clone, Component)]
    /// struct Name(&'static str);
    ///
    /// #[derive(Copy, Clone, Component)]
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
    pub fn extend_with<I, S>(&mut self, into_iter: I) -> Vec<Entity>
    where
        I: IntoIterator<Item = S>,
        S: ComponentSet,
    {
        into_iter
            .into_iter()
            .map(|set| self.create_with(set))
            .collect()
    }

    /// Returns `true` if the world contains the entity.
    /// Returns `false` if provided entity was destroyed in this world.
    ///
    /// # Examples
    ///
    /// ```
    /// # use toucan_ecs::prelude::*;
    /// let mut world = World::new();
    ///
    /// let entity = world.create();
    /// assert!(world.contains(entity));
    ///
    /// world.destroy(entity);
    /// assert!(!world.contains(entity));
    /// ```
    pub fn contains(&self, entity: Entity) -> bool {
        self.entities.contains(entity)
    }

    /// Returns `true` if the world has resource of generic type.
    ///
    /// # Examples
    ///
    /// ```
    /// # use toucan_ecs::prelude::*;
    /// struct Resource(u32);
    ///
    /// let mut world = World::new();
    ///
    /// world.create_resource(Resource(42));
    /// assert!(world.contains_resource::<Resource>());
    /// ```
    #[cfg(feature = "resource")]
    #[cfg_attr(docsrs, doc(cfg(feature = "resource")))]
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
    /// # use toucan_ecs::prelude::*;
    /// let mut world = World::new();
    ///
    /// let entity = world.create();
    /// world.destroy(entity);
    /// assert!(!world.contains(entity));
    /// ```
    pub fn destroy(&mut self, entity: Entity) {
        self.remove_all(entity);
        self.entities.destroy(entity);
    }

    /// Destroys the resource of generic type and removes it from the world.
    ///
    /// # Examples
    ///
    /// ```
    /// # use toucan_ecs::prelude::*;
    /// struct Resource(u32);
    ///
    /// let mut world = World::new();
    ///
    /// world.create_resource(Resource(42));
    /// world.destroy_resource::<Resource>();
    /// assert!(!world.contains_resource::<Resource>());
    /// ```
    #[cfg(feature = "resource")]
    #[cfg_attr(docsrs, doc(cfg(feature = "resource")))]
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
    /// # use toucan_ecs::prelude::*;
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
        self.entities.is_empty() && self.resources.is_empty()
    }

    #[cfg(not(feature = "resource"))]
    #[inline(always)]
    fn cfg_is_empty(&self) -> bool {
        self.entities.is_empty()
    }

    /// Clears this world, destroying all resources, all entities and their data.
    ///
    /// # Examples
    ///
    /// ```
    /// # use toucan_ecs::prelude::*;
    /// let mut world = World::new();
    ///
    /// world.extend(100);
    /// assert!(!world.is_empty());
    ///
    /// world.clear();
    /// assert!(world.is_empty());
    /// ```
    pub fn clear(&mut self) {
        self.entities.clear();
        self.components.clear();
        #[cfg(feature = "resource")]
        self.resources.clear();
    }

    /// Registers new type of component to be stored in the world.
    ///
    /// # Examples
    ///
    /// ```
    /// # use toucan_ecs::prelude::*;
    /// #[derive(Copy, Clone, Component)]
    /// struct Name(&'static str);
    ///
    /// #[derive(Copy, Clone, Component)]
    /// struct ID(u32);
    ///
    /// let mut world = World::new();
    /// world.register::<Name>();
    /// world.register::<ID>();
    /// ```
    pub fn register<C>(&mut self)
    where
        C: Component,
    {
        self.components.register::<C>();
    }

    /// Attaches one component or set of components to the entity.
    ///
    /// This function does not panic because it registers components' types automatically.
    ///
    /// # Examples
    ///
    /// ```
    /// # use toucan_ecs::prelude::*;
    /// #[derive(Copy, Clone, Component)]
    /// struct Name(&'static str);
    ///
    /// #[derive(Copy, Clone, Component)]
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
        self.components.attach(entity, set)
    }

    /// Returns `true` if one component or set of components are attached to the entity.
    ///
    /// # Examples
    ///
    /// ```
    /// # use toucan_ecs::prelude::*;
    /// #[derive(Copy, Clone, Component)]
    /// struct Name(&'static str);
    ///
    /// #[derive(Copy, Clone, Component)]
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
        self.components.attached::<S>(entity)
    }

    /// Returns `true` if the entity does not exist or does not contain any data attached to it.
    ///
    /// # Examples
    ///
    /// ```
    /// # use toucan_ecs::prelude::*;
    /// let mut world = World::new();
    ///
    /// let entity = world.create();
    /// assert!(world.is_entity_empty(entity));
    /// ```
    pub fn is_entity_empty(&self, entity: Entity) -> bool {
        self.components.is_entity_empty(entity)
    }

    /// Removes one component or set of components from the entity.
    ///
    /// # Examples
    ///
    /// ```
    /// # use toucan_ecs::prelude::*;
    /// #[derive(Copy, Clone, Component)]
    /// struct Name(&'static str);
    ///
    /// #[derive(Copy, Clone, Component)]
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
        self.components.remove::<S>(entity);
    }

    /// Removes all attached components from the entity.
    /// It makes the entity effectively empty.
    ///
    /// To remove just a set of components from the entity,
    /// use [`remove`][World::remove()] associated functions.
    ///
    /// # Examples
    ///
    /// ```
    /// # use toucan_ecs::prelude::*;
    /// #[derive(Copy, Clone, Component)]
    /// struct Name(&'static str);
    ///
    /// #[derive(Copy, Clone, Component)]
    /// struct ID(u32);
    ///
    /// let mut world = World::new();
    ///
    /// let entity = world.create_with((Name("Hello, World"), ID(42)));
    /// world.remove_all(entity);
    /// assert!(world.is_entity_empty(entity));
    /// ```
    pub fn remove_all(&mut self, entity: Entity) {
        self.components.remove_all(entity);
    }

    /// Retrieves the shared borrow for the component of one type attached to the entity.
    /// Returns [`None`](Option::None) if component is not attached to the entity.
    ///
    /// # Examples
    ///
    /// ```
    /// # use toucan_ecs::prelude::*;
    /// #[derive(Copy, Clone, Component, Eq, PartialEq, Debug)]
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
        self.components.get::<C>(entity)
    }

    /// Retrieves the unique borrow for the component of one type attached to the entity.
    /// Returns [`None`](Option::None) if component is not attached to the entity.
    ///
    /// # Examples
    ///
    /// ```
    /// # use toucan_ecs::prelude::*;
    /// #[derive(Copy, Clone, Component, Eq, PartialEq, Debug)]
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
        self.components.get_mut::<C>(entity)
    }

    /// Retrieves the shared borrow of the generic resource type.
    ///
    /// # Examples
    ///
    /// ```
    /// # use toucan_ecs::prelude::*;
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
    #[cfg_attr(docsrs, doc(cfg(feature = "resource")))]
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
    /// # use toucan_ecs::prelude::*;
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
    #[cfg_attr(docsrs, doc(cfg(feature = "resource")))]
    pub fn get_resource_mut<R>(&mut self) -> Option<&mut R>
    where
        R: Resource,
    {
        self.resources.get_mut::<R>()
    }

    /// Creates a [view](ViewOne) of the component type.
    ///
    /// This iterator will return [entities](Entity) and their shared borrows
    /// of components. Only entities that has that type of component will be returned.
    ///
    /// More complex views can be constructed with [`view`][World::view()] associated function.
    ///
    /// # Examples
    ///
    /// ```
    /// # use toucan_ecs::prelude::*;
    /// #[derive(Copy, Clone, Component, Debug)]
    /// struct Name(&'static str);
    ///
    /// let world = World::new();
    ///
    /// for (_, component) in world.view_one::<Name>() {
    ///     println!("component: {:?}", *component)
    /// }
    /// ```
    pub fn view_one<C>(&self) -> ViewOne<C>
    where
        C: Component,
    {
        let storage = self.components.get_storage::<C>();
        ViewOne::new(storage)
    }

    /// Creates a [view](ViewOneMut) of the component type.
    ///
    /// This iterator will return [entities](Entity) and their unique borrows
    /// of components. Only entities that has that type of component will be returned.
    ///
    /// Consider using [systems](crate::system::System)
    /// to mutate multiple components attached to the entity.
    ///
    /// # Examples
    ///
    /// ```
    /// # use toucan_ecs::prelude::*;
    /// #[derive(Copy, Clone, Component, Debug)]
    /// struct Name(&'static str);
    ///
    /// let mut world = World::new();
    ///
    /// for (_, component) in world.view_one_mut::<Name>() {
    ///     component.0 = "Hello, World!";
    ///     println!("component: {:?}", *component)
    /// }
    /// ```
    pub fn view_one_mut<C>(&mut self) -> ViewOneMut<C>
    where
        C: Component,
    {
        let storage = self.components.get_storage_mut::<C>();
        ViewOneMut::new(storage)
    }

    /// Creates a [view](View) of the multiple component types.
    ///
    /// This iterator will return [entities](Entity) and their shared borrows of components.
    ///
    /// View will be constructed from the query which is determined by the generic type.
    /// Only entities that satisfies the query will be returned.
    ///
    /// # Examples
    ///
    /// ```
    /// # use toucan_ecs::prelude::*;
    /// #[derive(Copy, Clone, Component, Debug)]
    /// struct Name(&'static str);
    ///
    /// #[derive(Copy, Clone, Component, Debug)]
    /// struct ID(u32);
    ///
    /// let world = World::new();
    ///
    /// for (name, id) in world.view::<(Option<&Name>, &ID)>() {
    ///     println!("name: {:?}, id: {:?}", name.as_deref(), *id)
    /// }
    /// ```
    pub fn view<'data, Q>(&'data self) -> View<'data, Q>
    where
        Q: Query<'data>,
    {
        let (entities, mut data) = self.split_refs();
        let entities = entities.iter();
        View::new(entities, &mut data)
    }

    /// Creates a [view](ViewMut) of the multiple component types.
    ///
    /// This iterator will return [entities](Entity) and their shared OR unique
    /// borrows of components.
    ///
    /// View will be constructed from the query which is determined by the generic type.
    /// Only entities that satisfies the query will be returned.
    ///
    /// # Panics
    ///
    /// This function will panic if provided query does not satisfies
    /// the first rule of references described in
    /// **References and Borrowing** section of [**Rust Book**][rust_book]:
    ///
    /// > - *At any given time, you can have either **one** mutable reference
    /// or **any** number of immutable references.*
    ///
    /// # Examples
    ///
    /// For this query function will panic:
    ///
    /// ```should_panic
    /// # use toucan_ecs::prelude::*;
    /// #[derive(Copy, Clone, Component, Debug)]
    /// struct ID(u32);
    ///
    /// let mut world = World::new();
    ///
    /// // immutable ID reference and mutable one of the same type are illegal
    /// for (id, mut mut_id) in world.view_mut::<(&ID, &mut ID)>() {
    ///     mut_id.0 += 10;
    ///     println!("unchanged id: {:?}", id)
    /// }
    /// ```
    ///
    /// But for this query it will not panic:
    ///
    /// ```
    /// # use toucan_ecs::prelude::*;
    /// #[derive(Copy, Clone, Component, Debug)]
    /// struct Name(&'static str);
    ///
    /// #[derive(Copy, Clone, Component, Debug)]
    /// struct ID(u32);
    ///
    /// let mut world = World::new();
    ///
    /// for (name, mut id) in world.view_mut::<(Option<&Name>, &mut ID)>() {
    ///     id.0 += 10;
    ///     println!("name: {:?}, id: {:?}", name.as_deref(), id)
    /// }
    /// ```
    ///
    /// [rust_book]: https://doc.rust-lang.org/book/ch04-02-references-and-borrowing.html#the-rules-of-references
    pub fn view_mut<'data, Q>(&'data mut self) -> ViewMut<'data, Q>
    where
        Q: QueryMut<'data>,
    {
        let (entities, mut data) = self.split_refs_mut();
        let entities = entities.iter();
        ViewMut::new(entities, &mut data)
    }

    pub(crate) fn components_mut(&mut self) -> &mut ComponentRegistry {
        &mut self.components
    }

    pub(crate) fn split_refs(&self) -> (&EntityRegistry, WorldRefs) {
        let entities = &self.entities;
        #[cfg(not(feature = "resource"))]
        let refs = {
            let storages = StorageRefs::from(&self.components);
            WorldRefs::new(storages)
        };
        #[cfg(feature = "resource")]
        let refs = {
            let storages = StorageRefs::from(&self.components);
            let resources = ResourceRefs::from(&self.resources);
            WorldRefs::new(storages, resources)
        };
        (entities, refs)
    }

    pub(crate) fn split_refs_mut(&mut self) -> (&EntityRegistry, WorldRefs) {
        let entities = &self.entities;
        #[cfg(not(feature = "resource"))]
        let refs = {
            let storages = StorageRefs::from(&mut self.components);
            WorldRefs::new(storages)
        };
        #[cfg(feature = "resource")]
        let refs = {
            let storages = StorageRefs::from(&mut self.components);
            let resources = ResourceRefs::from(&mut self.resources);
            WorldRefs::new(storages, resources)
        };
        (entities, refs)
    }
}
