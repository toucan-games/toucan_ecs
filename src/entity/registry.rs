use std::collections::HashMap;

use slotmap::dense::Keys;
use slotmap::DenseSlotMap;

use crate::component::{
    set::ComponentSet,
    storage::{DefaultStorage, Storage},
    type_id::ComponentTypeId,
};
use crate::{Component, Entity, Entry, Ref, RefMut};

use super::view::{SharedViewable, View, ViewMut, ViewOne, ViewOneMut, Viewable};

/// Registry of the entities and all the data attached to them.
///
/// Use this to [create][`Registry::create`] and [destroy][`Registry::destroy`] entities,
/// [attach][`Registry::attach`] and [remove][`Registry::remove`] components' data of the entity,
/// [create][`Registry::entry`] entry for the entity,
/// [view][`Registry::view`] each component separately or group of components together.
///
/// Registry can be used the same way as `World` in some other ECS libraries.
#[derive(Default)]
pub struct Registry {
    entities: DenseSlotMap<Entity, ()>,
    extended_entities: Vec<Entity>,
    storages: HashMap<ComponentTypeId, Box<dyn Storage + Send + Sync + 'static>>,
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
            extended_entities: Vec::new(),
            storages: HashMap::new(),
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
    /// assert!(registry.is_entity_empty(entity));
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
    /// assert!(!registry.is_entity_empty(entity));
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
    /// assert!(!registry.is_entity_empty(entity));
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
    /// assert!(registry.is_entity_empty(entity));
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
    /// assert!(!registry.is_entity_empty(entity));
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
    /// assert!(!registry.is_entity_empty(entity));
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

    /// Extends registry with provided count of newly created entities.
    /// Returns handles of newly created entities.
    ///
    /// # Examples
    ///
    /// ```
    /// # use toucan_ecs::Registry;
    /// let mut registry = Registry::new();
    ///
    /// let entities = registry.extend(10);
    /// assert_eq!(entities.len(), 10);
    /// ```
    pub fn extend(&mut self, count: u32) -> &[Entity] {
        self.extended_entities.clear();
        (0..count).for_each(|_| {
            let entity = self.create();
            self.extended_entities.push(entity);
        });
        self.extended_entities.as_slice()
    }

    /// Extends registry with collection of components to the newly created entities.
    /// Returns handles of newly created entities.
    ///
    /// # Examples
    ///
    /// ```
    /// # use toucan_ecs::Registry;
    /// #[derive(Copy, Clone)]
    /// struct ID(u32);
    ///
    /// let mut registry = Registry::new();
    ///
    /// let entities = registry.extend_with_one([ID(1), ID(2), ID(3), ID(4), ID(5)]);
    /// assert_eq!(entities.len(), 5);
    /// ```
    pub fn extend_with_one<I, C>(&mut self, into_iter: I) -> &[Entity]
    where
        I: IntoIterator<Item = C>,
        C: Component,
    {
        self.extended_entities.clear();
        let iter = into_iter.into_iter();
        iter.for_each(|component| {
            let entity = self.create_with_one(component);
            self.extended_entities.push(entity);
        });
        self.extended_entities.as_slice()
    }

    // noinspection SpellCheckingInspection
    /// Extends registry with collection of set of components to the newly created entities.
    /// Returns handles of newly created entities.
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
    /// let entities = registry.extend_with([
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
        self.extended_entities.clear();
        let iter = into_iter.into_iter();
        iter.for_each(|set| {
            let entity = self.create_with(set);
            self.extended_entities.push(entity);
        });
        self.extended_entities.as_slice()
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

    /// Returns `true` if registry does not contain any entity.
    ///
    /// # Examples
    ///
    /// ```
    /// # use toucan_ecs::Registry;
    /// let mut registry = Registry::new();
    /// assert!(registry.is_empty());
    ///
    /// let _ = registry.create();
    /// assert!(!registry.is_empty());
    /// ```
    pub fn is_empty(&self) -> bool {
        self.entities.is_empty()
    }

    /// Clears this registry, destroying all entities and their data.
    ///
    /// # Examples
    ///
    /// ```
    /// # use toucan_ecs::Registry;
    /// let mut registry = Registry::new();
    ///
    /// registry.extend(100);
    /// assert!(!registry.is_empty());
    ///
    /// registry.clear();
    /// assert!(registry.is_empty());
    /// ```
    pub fn clear(&mut self) {
        self.entities.clear();
        self.extended_entities.clear();
        self.storages
            .values_mut()
            .for_each(|storage| storage.clear());
    }

    /// Registers new type of component to be stored in the registry.
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
        if !self.has_storage::<C>() {
            self.create_storage::<C>();
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
        let storage = self.get_storage_mut().unwrap();
        storage.attach(entity, component);
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
        let storage = self.get_storage::<C>();
        storage
            .map(|storage| storage.attached(entity))
            .unwrap_or(false)
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

    /// Returns `true` if the entity does not exist or does not contain any data attached to it.
    ///
    /// # Examples
    ///
    /// ```
    /// # use toucan_ecs::Registry;
    /// let mut registry = Registry::new();
    ///
    /// let entity = registry.create();
    /// assert!(registry.is_entity_empty(entity));
    /// ```
    pub fn is_entity_empty(&self, entity: Entity) -> bool {
        self.storages
            .values()
            .all(|storage| !storage.attached(entity))
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
        let storage = self.get_storage_mut::<C>();
        if let Some(storage) = storage {
            storage.remove(entity)
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
    /// assert!(registry.is_entity_empty(entity));
    /// ```
    pub fn remove_all(&mut self, entity: Entity) {
        self.storages
            .values_mut()
            .for_each(|storage| storage.remove(entity))
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
        let storage = self.get_storage::<C>()?;
        storage.get(entity)
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
        let storage = self.get_storage_mut::<C>()?;
        storage.get_mut(entity)
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
    /// for component in registry.view_one::<Name>() {
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
    /// for mut component in registry.view_one_mut::<Name>() {
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
    /// for (name, id) in registry.view::<(Option<&Name>, &ID)>() {
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
    /// for (name, mut id) in registry.view_mut::<(Option<&Name>, &mut ID)>() {
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

    pub(super) fn get_storage<C>(&self) -> Option<&DefaultStorage<C>>
    where
        C: Component,
    {
        let type_id = ComponentTypeId::of::<C>();
        let storage = self.storages.get(&type_id)?;
        let storage = storage
            .as_ref()
            .as_any_ref()
            .downcast_ref()
            .expect("downcast error");
        Some(storage)
    }

    fn get_storage_mut<C>(&mut self) -> Option<&mut DefaultStorage<C>>
    where
        C: Component,
    {
        let type_id = ComponentTypeId::of::<C>();
        let storage = self.storages.get_mut(&type_id)?;
        let storage = storage
            .as_mut()
            .as_any_mut()
            .downcast_mut()
            .expect("downcast error");
        Some(storage)
    }

    fn has_storage<C>(&self) -> bool
    where
        C: Component,
    {
        let type_id = ComponentTypeId::of::<C>();
        self.storages.contains_key(&type_id)
    }

    fn create_storage<C>(&mut self)
    where
        C: Component,
    {
        let type_id = ComponentTypeId::of::<C>();
        let storage = DefaultStorage::<C>::new();
        self.storages.insert(type_id, Box::new(storage));
    }
}
