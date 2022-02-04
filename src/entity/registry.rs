use std::collections::HashMap;

use slotmap::dense::Keys;
use slotmap::DenseSlotMap;

use crate::component::{
    pool::{ComponentPool, Pool},
    set::ComponentSet,
    type_id::ComponentTypeId,
};
use crate::{Component, Entity, Entry, Ref, RefMut};

use super::view::{SharedViewable, View, ViewMut, ViewOne, ViewOneMut, Viewable};

/// Storage of the entities and all the data attached to them.
///
/// Use this to [create][`Registry::create`] and [destroy][`Registry::destroy`] entities,
/// [attach][`Registry::attach`] and [remove][`Registry::remove`] components' data of the entity,
/// [create][`Registry::entry`] entry for the entity,
/// [view][`Registry::view`] each component separately or group of components together.
///
/// Registry can be used the same way as `World` in some other ECS libraries.
pub struct Registry {
    entities: DenseSlotMap<Entity, ()>,
    pools: HashMap<ComponentTypeId, Box<dyn Pool + Send + Sync + 'static>>,
}

impl Registry {
    /// Creates an empty registry with no entities and no data.
    ///
    /// # Examples
    ///
    /// ```
    /// # use toucan_ecs::Registry;
    /// let registry = Registry::new();
    /// ```
    pub fn new() -> Self {
        Self {
            entities: DenseSlotMap::with_key(),
            pools: HashMap::new(),
        }
    }

    /// Creates new entity with no data attached to it.
    ///
    /// To attach some data to the entity, use [`attach`][`Registry::attach`]
    /// or [`attach_one`][`Registry::attach_one`] associated functions.
    ///
    /// # Examples
    ///
    /// ```
    /// # use toucan_ecs::Registry;
    /// let mut registry = Registry::new();
    ///
    /// let entity = registry.create();
    /// assert!(registry.contains(entity));
    /// ```
    pub fn create(&mut self) -> Entity {
        self.entities.insert(())
    }

    /// Creates new entity with one component attached to it.
    ///
    /// This can be done by hand with [`attach_one`][`Registry::attach_one`] associated function.
    ///
    /// # Examples
    ///
    /// ```
    /// # use toucan_ecs::Registry;
    /// #[derive(Copy, Clone)]
    /// struct Name(&'static str);
    ///
    /// let mut registry = Registry::new();
    /// let entity = registry.create_with_one(Name("Hello, World"));
    /// assert!(registry.contains(entity));
    /// ```
    pub fn create_with_one<C>(&mut self, component: C) -> Entity
    where
        C: Component,
    {
        let entity = self.create();
        self.attach_one(entity, component);
        entity
    }

    /// Creates new entity with set of components attached to it.
    ///
    /// This can be done by hand with [`attach`][`Registry::attach`] associated function.
    ///
    /// # Examples
    ///
    /// ```
    /// # use toucan_ecs::Registry;
    /// #[derive(Copy, Clone)]
    /// struct Name(&'static str);
    ///
    /// #[derive(Copy, Clone)]
    /// struct ID(u32);
    ///
    /// let mut registry = Registry::new();
    /// let entity = registry.create_with((Name("Hello, World"), ID(42)));
    /// assert!(registry.contains(entity));
    /// ```
    pub fn create_with<S>(&mut self, set: S) -> Entity
    where
        S: ComponentSet,
    {
        let entity = self.create();
        self.attach(entity, set);
        entity
    }

    /// Creates new [entry][`Entry`] for the newly created entity.
    ///
    /// # Examples
    ///
    /// ```
    /// # use toucan_ecs::Registry;
    /// let mut registry = Registry::new();
    ///
    /// let entry = registry.create_entry();
    /// let entity = entry.entity();
    /// assert!(registry.contains(entity));
    /// ```
    pub fn create_entry(&mut self) -> Entry {
        let entity = self.create();
        Entry::new(entity, self)
    }

    /// Creates [entry][`Entry`] for the newly created entity with one component attached to it.
    ///
    /// This can be done by hand with [`Entry::attach_one`] associated function.
    ///
    /// # Examples
    ///
    /// ```
    /// # use toucan_ecs::Registry;
    /// #[derive(Copy, Clone)]
    /// struct Name(&'static str);
    ///
    /// let mut registry = Registry::new();
    /// let entry = registry.create_entry_with_one(Name("Hello, World"));
    ///
    /// let entity = entry.entity();
    /// assert!(registry.contains(entity));
    /// ```
    pub fn create_entry_with_one<C>(&mut self, component: C) -> Entry
    where
        C: Component,
    {
        let entity = self.create_with_one(component);
        Entry::new(entity, self)
    }

    /// Creates [entry][`Entry`] for the newly created entity with set of components attached to it.
    ///
    /// This can be done by hand with [`Entry::attach`] associated function.
    ///
    /// # Examples
    ///
    /// ```
    /// # use toucan_ecs::Registry;
    /// #[derive(Copy, Clone)]
    /// struct Name(&'static str);
    ///
    /// #[derive(Copy, Clone)]
    /// struct ID(u32);
    ///
    /// let mut registry = Registry::new();
    /// let entry = registry.create_entry_with((Name("Hello, World"), ID(42)));
    ///
    /// let entity = entry.entity();
    /// assert!(registry.contains(entity));
    /// ```
    pub fn create_entry_with<S>(&mut self, set: S) -> Entry
    where
        S: ComponentSet,
    {
        let entity = self.create_with(set);
        Entry::new(entity, self)
    }

    /// Create [entry][`Entry`] for the provided entity.
    ///
    /// Returns [`None`][`Option::None`] if the provided entity was previously destroyed.
    ///
    /// # Examples
    ///
    /// ```
    /// # use toucan_ecs::Registry;
    /// let mut registry = Registry::new();
    ///
    /// let entity = registry.create();
    /// assert!(registry.entry(entity).is_some());
    ///
    /// registry.destroy(entity);
    /// assert!(registry.entry(entity).is_none());
    /// ```
    pub fn entry(&mut self, entity: Entity) -> Option<Entry> {
        self.contains(entity).then(|| Entry::new(entity, self))
    }

    /// Returns `true` if the registry contains the entity.
    ///
    /// # Examples
    ///
    /// ```
    /// # use toucan_ecs::Registry;
    /// let mut registry = Registry::new();
    ///
    /// let entity = registry.create();
    /// assert!(registry.contains(entity));
    ///
    /// registry.destroy(entity);
    /// assert!(!registry.contains(entity));
    /// ```
    pub fn contains(&self, entity: Entity) -> bool {
        self.entities.contains_key(entity)
    }

    /// Destroys the entity and removes all its attached components.
    ///
    /// # Examples
    ///
    /// ```
    /// # use toucan_ecs::Registry;
    /// let mut registry = Registry::new();
    ///
    /// let entity = registry.create();
    /// registry.destroy(entity);
    /// assert!(!registry.contains(entity));
    /// ```
    pub fn destroy(&mut self, entity: Entity) {
        self.remove_all(entity);
        self.entities.remove(entity);
    }

    /// Registers new type of component to be stored in the registry.
    ///
    /// # Panics
    ///
    /// Attempt to attach unregistered component to the entity will result in panic.
    ///
    /// # Examples
    ///
    /// ```
    /// # use toucan_ecs::Registry;
    /// #[derive(Copy, Clone)]
    /// struct ID(u32);
    ///
    /// let mut registry = Registry::new();
    /// registry.register::<&'static str>();
    /// registry.register::<ID>();
    /// ```
    pub fn register<C>(&mut self)
    where
        C: Component,
    {
        let pool = self.get_pool_mut::<C>();
        if pool.is_none() {
            self.create_pool::<C>();
        }
    }

    /// Attaches exactly one component to the entity.
    ///
    /// This function does not panic because it registers component type automatically.
    ///
    /// To attach multiple components of different types to the entity at once,
    /// use [`attach`][`Registry::attach`] associated function.
    ///
    /// # Examples
    ///
    /// ```
    /// # use toucan_ecs::Registry;
    /// #[derive(Copy, Clone, Eq, PartialEq, Debug)]
    /// struct Name(&'static str);
    ///
    /// let mut registry = Registry::new();
    ///
    /// let entity = registry.create();
    /// registry.attach_one(entity, Name("Hello, World"));
    /// assert_eq!(registry.get(entity).as_deref(), Some(&Name("Hello, World")));
    /// ```
    pub fn attach_one<C>(&mut self, entity: Entity, component: C)
    where
        C: Component,
    {
        self.register::<C>();
        let pool = self.get_pool_mut().unwrap();
        pool.attach(entity, component);
    }

    /// Attaches set of components to the entity.
    ///
    /// This function does not panic because it registers components' types automatically.
    ///
    /// To attach component of exactly one type to the entity,
    /// use [`attach_one`][`Registry::attach_one`] associated function.
    ///
    /// # Examples
    ///
    /// ```
    /// # use toucan_ecs::Registry;
    /// #[derive(Copy, Clone)]
    /// struct Name(&'static str);
    ///
    /// #[derive(Copy, Clone)]
    /// struct ID(u32);
    ///
    /// let mut registry = Registry::new();
    ///
    /// let entity = registry.create();
    /// registry.attach(entity, (Name("Hello, World"), ID(42)));
    /// assert!(registry.attached::<(ID, Name)>(entity));
    /// ```
    pub fn attach<S>(&mut self, entity: Entity, set: S)
    where
        S: ComponentSet,
    {
        set.attach(self, entity)
    }

    /// Returns `true` if component of generic type is attached to the entity.
    ///
    /// To check if the entity has components of multiple types,
    /// use [`attached`][`Registry::attached`] associated function.
    ///
    /// # Examples
    ///
    /// ```
    /// # use toucan_ecs::Registry;
    /// #[derive(Copy, Clone)]
    /// struct Name(&'static str);
    ///
    /// let mut registry = Registry::new();
    ///
    /// let entity = registry.create();
    /// assert!(!registry.attached_one::<Name>(entity));
    ///
    /// registry.attach_one(entity, Name("Hello, World"));
    /// assert!(registry.attached_one::<Name>(entity));
    /// ```
    pub fn attached_one<C>(&self, entity: Entity) -> bool
    where
        C: Component,
    {
        let pool = self.get_pool::<C>();
        pool.map(|pool| pool.attached(entity)).unwrap_or(false)
    }

    /// Returns `true` if components in the generic set type are attached to the entity.
    ///
    /// To check if the entity has component of exactly one type,
    /// use [`attached_one`][`Registry::attached_one`] associated function.
    ///
    /// # Examples
    ///
    /// ```
    /// # use toucan_ecs::Registry;
    /// #[derive(Copy, Clone)]
    /// struct Name(&'static str);
    ///
    /// #[derive(Copy, Clone)]
    /// struct ID(u32);
    ///
    /// let mut registry = Registry::new();
    ///
    /// let entity = registry.create();
    /// assert!(!registry.attached::<(ID, Name)>(entity));
    ///
    /// registry.attach(entity, (Name("Hello, World"), ID(42)));
    /// assert!(registry.attached::<(Name, ID)>(entity));
    /// ```
    pub fn attached<S>(&self, entity: Entity) -> bool
    where
        S: ComponentSet,
    {
        S::attached(self, entity)
    }

    /// Removes component of one type from the entity.
    ///
    /// To remove components of multiple types from the entity at once,
    /// use [`remove`][`Registry::remove`] associated function.
    ///
    /// # Examples
    ///
    /// ```
    /// # use toucan_ecs::Registry;
    /// #[derive(Copy, Clone)]
    /// struct Name(&'static str);
    ///
    /// let mut registry = Registry::new();
    ///
    /// let entity = registry.create_with((Name("Hello, World"),));
    /// assert!(registry.attached_one::<Name>(entity));
    ///
    /// registry.remove_one::<Name>(entity);
    /// assert!(!registry.attached_one::<Name>(entity));
    /// ```
    pub fn remove_one<C>(&mut self, entity: Entity)
    where
        C: Component,
    {
        let pool = self.get_pool_mut::<C>();
        if let Some(pool) = pool {
            pool.remove(entity)
        }
    }

    /// Removes components of multiple types from the entity.
    ///
    /// To remove component of one type from the entity,
    /// use [`remove_one`][`Registry::remove_one`] associated function.
    ///
    /// # Examples
    ///
    /// ```
    /// # use toucan_ecs::Registry;
    /// #[derive(Copy, Clone)]
    /// struct Name(&'static str);
    ///
    /// #[derive(Copy, Clone)]
    /// struct ID(u32);
    ///
    /// let mut registry = Registry::new();
    ///
    /// let entity = registry.create_with((Name("Hello, World"), ID(42)));
    /// registry.remove::<(ID, Name)>(entity);
    /// assert!(!registry.attached::<(Name, ID)>(entity));
    /// ```
    pub fn remove<S>(&mut self, entity: Entity)
    where
        S: ComponentSet,
    {
        S::remove(self, entity)
    }

    /// Removes all attached components from the entity.
    /// It makes the entity effectively empty.
    ///
    /// To remove just a set of components from the entity,
    /// use [`remove_one`][`Registry::remove_one`] and [`remove`][`Registry::remove`]
    /// associated functions.
    ///
    /// # Examples
    ///
    /// ```
    /// # use toucan_ecs::Registry;
    /// #[derive(Copy, Clone)]
    /// struct Name(&'static str);
    ///
    /// #[derive(Copy, Clone)]
    /// struct ID(u32);
    ///
    /// let mut registry = Registry::new();
    ///
    /// let entity = registry.create_with((Name("Hello, World"), ID(42)));
    /// registry.remove_all(entity);
    /// assert!(!registry.attached::<(Name, ID)>(entity));
    /// ```
    pub fn remove_all(&mut self, entity: Entity) {
        self.pools.values_mut().for_each(|pool| pool.remove(entity))
    }

    /// Retrieves the [shared borrow][`Ref`] for the component of one type attached to the entity.
    /// Returns [`None`][`Option::None`] if component is not attached to the entity.
    ///
    /// Note that function would block current thread
    /// if the same instance of component will be retrieved more than once.
    ///
    /// # Examples
    ///
    /// ```
    /// # use toucan_ecs::Registry;
    /// #[derive(Copy, Clone, Eq, PartialEq, Debug)]
    /// struct Name(&'static str);
    ///
    /// let mut registry = Registry::new();
    ///
    /// let entity = registry.create_with((Name("Hello, World"),));
    /// let name = registry.get::<Name>(entity).unwrap();
    /// assert_eq!(*name, Name("Hello, World"));
    /// ```
    pub fn get<C>(&self, entity: Entity) -> Option<Ref<C>>
    where
        C: Component,
    {
        let pool = self.get_pool::<C>()?;
        pool.get(entity)
    }

    /// Retrieves the [unique borrow][`RefMut`] for the component of one type attached to the entity.
    /// Returns [`None`][`Option::None`] if component is not attached to the entity.
    ///
    /// Note that function would block current thread
    /// if the same instance of component will be retrieved more than once.
    ///
    /// # Examples
    ///
    /// ```
    /// # use toucan_ecs::Registry;
    /// #[derive(Copy, Clone, Eq, PartialEq, Debug)]
    /// struct Name(&'static str);
    ///
    /// let mut registry = Registry::new();
    ///
    /// let entity = registry.create_with((Name("Hello, World"),));
    /// let mut name = registry.get_mut::<Name>(entity).unwrap();
    /// name.0 = "This name was changed";
    /// assert_ne!(*name, Name("Hello, World"));
    /// assert_eq!(*name, Name("This name was changed"));
    /// ```
    pub fn get_mut<C>(&mut self, entity: Entity) -> Option<RefMut<C>>
    where
        C: Component,
    {
        let pool = self.get_pool_mut::<C>()?;
        pool.get_mut(entity)
    }

    /// Creates a [view][`ViewOne`] of the one component type.
    ///
    /// This iterator will return [entities][`Entity`] and their [shared borrows][`Ref`]
    /// of components. Only entities that has that type of component will be returned.
    ///
    /// More complex views can be constructed with
    /// [view][`Registry::view`] associated function.
    ///
    /// # Examples
    ///
    /// ```
    /// # use toucan_ecs::Registry;
    /// #[derive(Copy, Clone, Debug)]
    /// struct Name(&'static str);
    ///
    /// let registry = Registry::new();
    ///
    /// for (entity, component) in registry.view_one::<Name>() {
    ///     println!("component: {:?}", *component)
    /// }
    /// ```
    pub fn view_one<C>(&self) -> ViewOne<C>
    where
        C: Component,
    {
        ViewOne::new(self)
    }

    // noinspection SpellCheckingInspection
    /// Creates a [view][`ViewOne`] of the one component type.
    ///
    /// This iterator will return [entities][`Entity`] and their [unique borrows][`RefMut`]
    /// of components. Only entities that has that type of component will be returned.
    ///
    /// More complex views can be constructed with
    /// [view][`Registry::view`] associated function.
    ///
    /// # Examples
    ///
    /// ```
    /// # use toucan_ecs::Registry;
    /// #[derive(Copy, Clone, Debug)]
    /// struct Name(&'static str);
    ///
    /// let mut registry = Registry::new();
    ///
    /// for (entity, mut component) in registry.view_one_mut::<Name>() {
    ///     component.0 = "Привет, Мир";
    ///     println!("component: {:?}", *component)
    /// }
    /// ```
    pub fn view_one_mut<C>(&mut self) -> ViewOneMut<C>
    where
        C: Component,
    {
        ViewOneMut::new(self)
    }

    /// Creates a [view][`View`] of the multiple component types.
    ///
    /// This iterator will return [entities][`Entity`] and their shared borrows (not only [`Ref`])
    /// of components.
    ///
    /// View will be constructed from the query which is determined by the generic type.
    /// Only entities that satisfies the query will be returned.
    ///
    /// # Examples
    ///
    /// ```
    /// # use toucan_ecs::Registry;
    /// #[derive(Copy, Clone, Debug)]
    /// struct Name(&'static str);
    ///
    /// #[derive(Copy, Clone, Debug)]
    /// struct ID(u32);
    ///
    /// let registry = Registry::new();
    ///
    /// for (entity, (name, id)) in registry.view::<(Option<&Name>, &ID)>() {
    ///     println!("name: {:?}, id: {:?}", name.as_deref(), *id)
    /// }
    /// ```
    pub fn view<'data, V>(&'data self) -> View<'data, V>
    where
        V: SharedViewable<'data>,
    {
        View::new(self)
    }

    /// Creates a [view][`View`] of the multiple component types.
    ///
    /// This iterator will return [entities][`Entity`] and their shared OR unique borrows
    /// (not only [`Ref`] or [`RefMut`]) of components.
    ///
    /// View will be constructed from the query which is determined by the generic type.
    /// Only entities that satisfies the query will be returned.
    ///
    /// # Examples
    ///
    /// ```
    /// # use toucan_ecs::Registry;
    /// #[derive(Copy, Clone, Debug)]
    /// struct Name(&'static str);
    ///
    /// #[derive(Copy, Clone, Debug)]
    /// struct ID(u32);
    ///
    /// let mut registry = Registry::new();
    ///
    /// for (entity, (name, mut id)) in registry.view_mut::<(Option<&Name>, &mut ID)>() {
    ///     id.0 += 10;
    ///     println!("name: {:?}, id: {:?}", name.as_deref(), *id)
    /// }
    /// ```
    pub fn view_mut<'data, V>(&'data mut self) -> ViewMut<'data, V>
    where
        V: Viewable<'data>,
    {
        ViewMut::new(self)
    }

    pub(super) fn entities(&self) -> Keys<Entity, ()> {
        self.entities.keys()
    }

    pub(super) fn get_pool<C>(&self) -> Option<&ComponentPool<C>>
    where
        C: Component,
    {
        let type_id = ComponentTypeId::of::<C>();
        let pool = self.pools.get(&type_id)?;
        let pool = pool
            .as_ref()
            .as_any_ref()
            .downcast_ref()
            .expect("downcast error");
        Some(pool)
    }

    fn get_pool_mut<C>(&mut self) -> Option<&mut ComponentPool<C>>
    where
        C: Component,
    {
        let type_id = ComponentTypeId::of::<C>();
        let pool = self.pools.get_mut(&type_id)?;
        let pool = pool
            .as_mut()
            .as_any_mut()
            .downcast_mut()
            .expect("downcast error");
        Some(pool)
    }

    fn create_pool<C>(&mut self)
    where
        C: Component,
    {
        let type_id = ComponentTypeId::of::<C>();
        let pool = ComponentPool::<C>::new();
        self.pools.insert(type_id, Box::new(pool));
    }
}
