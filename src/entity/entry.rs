use crate::component::set::ComponentSet;
use crate::{Component, Entity, Ref, RefMut, Registry};

/// Entry of the specific [entity][`Entity`].
///
/// Use this struct to simplify access to the entity so
/// you don't have to provide it each time to retrieve something,
/// you can do it only once.
///
/// You can retrieve this
/// from [`Registry::create_entry`] to create new entity and easily access it
/// or from [`Registry::entry`] if an entity was created earlier.
pub struct Entry<'r> {
    entity: Entity,
    registry: &'r mut Registry,
}

impl<'r> Entry<'r> {
    pub(super) fn new(entity: Entity, registry: &'r mut Registry) -> Self {
        Entry { entity, registry }
    }

    /// Attaches exactly one component to the entity.
    ///
    /// This function does not panic because it registers component type automatically.
    ///
    /// To attach multiple components of different types to the entity at once,
    /// use [`attach`][`Entry::attach`] associated function.
    ///
    /// # Examples
    ///
    /// ```
    /// # use toucan_ecs::{Registry, Entry};
    /// # let mut registry = Registry::new();
    /// #[derive(Copy, Clone, Eq, PartialEq, Debug)]
    /// struct Name(&'static str);
    ///
    /// let mut entry = registry.create_entry();
    /// entry.attach_one(Name("Hello, World"));
    /// assert_eq!(entry.get().as_deref(), Some(&Name("Hello, World")));
    /// ```
    pub fn attach_one<C>(&mut self, component: C)
    where
        C: Component,
    {
        self.registry.attach_one(self.entity, component);
    }

    /// Attaches set of components to the entity.
    ///
    /// This function does not panic because it registers components' types automatically.
    ///
    /// To attach component of exactly one type to the entity,
    /// use [`attach_one`][`Entry::attach_one`] associated function.
    ///
    /// # Examples
    ///
    /// ```
    /// # use toucan_ecs::{Registry, Entry};
    /// # let mut registry = Registry::new();
    /// #[derive(Copy, Clone)]
    /// struct Name(&'static str);
    ///
    /// #[derive(Copy, Clone)]
    /// struct ID(u32);
    ///
    /// let mut entry = registry.create_entry();
    /// entry.attach((Name("Hello, World"), ID(42)));
    /// assert!(entry.attached::<(ID, Name)>());
    /// ```
    pub fn attach<S>(&mut self, set: S)
    where
        S: ComponentSet,
    {
        self.registry.attach(self.entity, set)
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
    /// # use toucan_ecs::{Registry, Entry};
    /// # let mut registry = Registry::new();
    /// #[derive(Copy, Clone, Eq, PartialEq, Debug)]
    /// struct Name(&'static str);
    ///
    /// let mut entry = registry.create_entry();
    /// entry.attach_one(Name("Hello, World"));
    ///
    /// let name = entry.get::<Name>().unwrap();
    /// assert_eq!(*name, Name("Hello, World"));
    /// ```
    pub fn get<C>(&'r self) -> Option<Ref<'r, C>>
    where
        C: Component,
    {
        self.registry.get(self.entity)
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
    /// # use toucan_ecs::{Registry, Entry};
    /// # let mut registry = Registry::new();
    /// #[derive(Copy, Clone, Eq, PartialEq, Debug)]
    /// struct Name(&'static str);
    ///
    /// let mut entry = registry.create_entry();
    /// entry.attach_one(Name("Hello, World"));
    ///
    /// let mut name = entry.get_mut::<Name>().unwrap();
    /// (*name).0 = "This name was changed";
    /// assert_ne!(*name, Name("Hello, World"));
    /// assert_eq!(*name, Name("This name was changed"));
    /// ```
    pub fn get_mut<C>(&'r mut self) -> Option<RefMut<'r, C>>
    where
        C: Component,
    {
        self.registry.get_mut(self.entity)
    }

    /// Removes component of one type from the entity.
    ///
    /// To remove components of multiple types from the entity at once,
    /// use [`remove`][`Entry::remove`] associated function.
    ///
    /// # Examples
    ///
    /// ```
    /// # use toucan_ecs::{Registry, Entry};
    /// # let mut registry = Registry::new();
    /// #[derive(Copy, Clone)]
    /// struct Name(&'static str);
    ///
    /// let mut entry = registry.create_entry();
    ///
    /// entry.attach_one(Name("Hello, World"));
    /// assert!(entry.attached_one::<Name>());
    ///
    /// entry.remove_one::<Name>();
    /// assert!(!entry.attached_one::<Name>());
    /// ```
    pub fn remove_one<C>(&mut self)
    where
        C: Component,
    {
        self.registry.remove_one::<C>(self.entity)
    }

    /// Removes components of multiple types from the entity.
    ///
    /// To remove component of one type from the entity,
    /// use [`remove_one`][`Entry::remove_one`] associated function.
    ///
    /// # Examples
    ///
    /// ```
    /// # use toucan_ecs::{Registry, Entry};
    /// # let mut registry = Registry::new();
    /// #[derive(Copy, Clone)]
    /// struct Name(&'static str);
    ///
    /// #[derive(Copy, Clone)]
    /// struct ID(u32);
    ///
    /// let mut entry = registry.create_entry();
    /// entry.attach((Name("Hello, World"), ID(42)));
    ///
    /// entry.remove::<(ID, Name)>();
    /// assert!(!entry.attached::<(Name, ID)>());
    /// ```
    pub fn remove<S>(&mut self)
    where
        S: ComponentSet,
    {
        self.registry.remove::<S>(self.entity)
    }

    /// Removes all attached components from the entity.
    /// It makes the entity effectively empty.
    ///
    /// To remove just a set of components from the entity,
    /// use [`remove_one`][`Entry::remove_one`] and [`remove`][`Entry::remove`]
    /// associated functions.
    ///
    /// # Examples
    ///
    /// ```
    /// # use toucan_ecs::{Registry, Entry};
    /// # let mut registry = Registry::new();
    /// #[derive(Copy, Clone)]
    /// struct Name(&'static str);
    ///
    /// #[derive(Copy, Clone)]
    /// struct ID(u32);
    ///
    /// let mut entry = registry.create_entry();
    /// entry.attach((Name("Hello, World"), ID(42)));
    ///
    /// entry.remove_all();
    /// assert!(!entry.attached::<(Name, ID)>());
    /// ```
    pub fn remove_all(&mut self) {
        self.registry.remove_all(self.entity)
    }

    /// Returns `true` if component of generic type is attached to the entity.
    ///
    /// To check if the entity has components of multiple types,
    /// use [`attached`][`Entry::attached`] associated function.
    ///
    /// # Examples
    ///
    /// ```
    /// # use toucan_ecs::{Registry, Entry};
    /// # let mut registry = Registry::new();
    /// #[derive(Copy, Clone)]
    /// struct Name(&'static str);
    ///
    /// let mut entry = registry.create_entry();
    /// assert!(!entry.attached_one::<Name>());
    ///
    /// entry.attach_one(Name("Hello, World"));
    /// assert!(entry.attached_one::<Name>());
    /// ```
    pub fn attached_one<C>(&self) -> bool
    where
        C: Component,
    {
        self.registry.attached_one::<C>(self.entity)
    }

    /// Returns `true` if components in the generic set type are attached to the entity.
    ///
    /// To check if the entity has component of exactly one type,
    /// use [`attached_one`][`Entry::attached_one`] associated function.
    ///
    /// # Examples
    ///
    /// ```
    /// # use toucan_ecs::{Registry, Entry};
    /// # let mut registry = Registry::new();
    /// #[derive(Copy, Clone)]
    /// struct Name(&'static str);
    ///
    /// #[derive(Copy, Clone)]
    /// struct ID(u32);
    ///
    /// let mut entry = registry.create_entry();
    /// assert!(!entry.attached::<(ID, Name)>());
    ///
    /// entry.attach((Name("Hello, World"), ID(42)));
    /// assert!(entry.attached::<(Name, ID)>());
    /// ```
    pub fn attached<S>(&self) -> bool
    where
        S: ComponentSet,
    {
        self.registry.attached::<S>(self.entity)
    }

    /// Destroys the entity and removes all its attached components.
    ///
    /// # Examples
    ///
    /// ```
    /// # use toucan_ecs::{Registry, Entry};
    /// # let mut registry = Registry::new();
    /// let mut entry = registry.create_entry();
    /// let entity = entry.entity();
    ///
    /// entry.destroy();
    /// assert!(!registry.contains(entity));
    /// ```
    pub fn destroy(self) {
        self.registry.destroy(self.entity)
    }

    /// Returns unique handle of the entity.
    ///
    /// # Examples
    ///
    /// ```
    /// # use toucan_ecs::{Registry, Entry};
    /// # let mut registry = Registry::new();
    /// let mut entry = registry.create_entry();
    /// let entity = entry.entity();
    /// assert!(registry.contains(entity));
    /// ```
    pub fn entity(&self) -> Entity {
        self.entity
    }
}
