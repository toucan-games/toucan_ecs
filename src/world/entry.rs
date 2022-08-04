use crate::component::{Component, ComponentSet};
use crate::entity::Entity;
use crate::world::World;

/// Entry of the specific [entity](Entity).
///
/// Use this struct to simplify access to the entity so
/// you don't have to provide it each time to retrieve something,
/// you can do it only once.
///
/// You can retrieve this from
/// [`World::create_entry`][crate::world::World::create_entry()] to create new entity and easily
/// access it or from [`World::entry`][crate::world::World::entry()]
/// if an entity was created earlier.
pub struct Entry<'data> {
    entity: Entity,
    world: &'data mut World,
}

impl<'data> Entry<'data> {
    pub(crate) fn new(entity: Entity, world: &'data mut World) -> Self {
        Entry { entity, world }
    }

    /// Destroys the entity and removes all its attached components.
    ///
    /// # Examples
    ///
    /// ```
    /// # use toucan_ecs::prelude::*;
    /// # let mut world = World::new();
    /// let mut entry = world.create_entry();
    /// let entity = entry.entity();
    ///
    /// entry.destroy();
    /// assert!(!world.contains(entity));
    /// ```
    pub fn destroy(self) {
        self.world.destroy(self.entity)
    }

    /// Returns unique handle of the entity.
    ///
    /// # Examples
    ///
    /// ```
    /// # use toucan_ecs::prelude::*;
    /// # let mut world = World::new();
    /// let mut entry = world.create_entry();
    /// let entity = entry.entity();
    /// assert!(world.contains(entity));
    /// ```
    pub fn entity(&self) -> Entity {
        self.entity
    }

    /// Returns `true` if the entity does not exist or does not contain any data attached to it.
    ///
    /// # Examples
    ///
    /// ```
    /// # use toucan_ecs::prelude::*;
    /// # let mut world = World::new();
    /// let mut entry = world.create_entry();
    /// assert!(entry.is_empty());
    /// ```
    pub fn is_empty(&self) -> bool {
        self.world.is_entity_empty(self.entity)
    }

    /// Attaches one component or set of components to the entity.
    ///
    /// This function does not panic because it registers components' types automatically.
    ///
    /// # Examples
    ///
    /// ```
    /// # use toucan_ecs::prelude::*;
    /// # let mut world = World::new();
    /// #[derive(Copy, Clone, Component)]
    /// struct Name(&'static str);
    ///
    /// #[derive(Copy, Clone, Component)]
    /// struct ID(u32);
    ///
    /// let mut entry = world.create_entry_with((Name("Hello, World"), ID(42)));
    /// assert!(entry.attached::<(ID, Name)>());
    /// ```
    pub fn attach<S>(&mut self, set: S)
    where
        S: ComponentSet,
    {
        self.world.attach(self.entity, set)
    }

    /// Returns `true` if one component or set of components are attached to the entity.
    ///
    /// # Examples
    ///
    /// ```
    /// # use toucan_ecs::prelude::*;
    /// # let mut world = World::new();
    /// #[derive(Copy, Clone, Component)]
    /// struct Name(&'static str);
    ///
    /// #[derive(Copy, Clone, Component)]
    /// struct ID(u32);
    ///
    /// let mut entry = world.create_entry();
    /// assert!(!entry.attached::<(ID, Name)>());
    ///
    /// entry.attach((Name("Hello, World"), ID(42)));
    /// assert!(entry.attached::<(Name, ID)>());
    /// ```
    pub fn attached<S>(&self) -> bool
    where
        S: ComponentSet,
    {
        self.world.attached::<S>(self.entity)
    }

    /// Removes one component or set of components from the entity.
    ///
    /// # Examples
    ///
    /// ```
    /// # use toucan_ecs::prelude::*;
    /// # let mut world = World::new();
    /// #[derive(Copy, Clone, Component)]
    /// struct Name(&'static str);
    ///
    /// #[derive(Copy, Clone, Component)]
    /// struct ID(u32);
    ///
    /// let mut entry = world.create_entry_with((Name("Hello, World"), ID(42)));
    /// entry.remove::<(ID, Name)>();
    /// assert!(!entry.attached::<(Name, ID)>());
    /// ```
    pub fn remove<S>(&mut self)
    where
        S: ComponentSet,
    {
        self.world.remove::<S>(self.entity)
    }

    /// Removes all attached components from the entity.
    /// It makes the entity effectively empty.
    ///
    /// To remove just a set of components from the entity,
    /// use [`remove`][Entry::remove()] associated functions.
    ///
    /// # Examples
    ///
    /// ```
    /// # use toucan_ecs::prelude::*;
    /// # let mut world = World::new();
    /// #[derive(Copy, Clone, Component)]
    /// struct Name(&'static str);
    ///
    /// #[derive(Copy, Clone, Component)]
    /// struct ID(u32);
    ///
    /// let mut entry = world.create_entry_with((Name("Hello, World"), ID(42)));
    /// entry.remove_all();
    /// assert!(!entry.attached::<(Name, ID)>());
    /// ```
    pub fn remove_all(&mut self) {
        self.world.remove_all(self.entity)
    }

    /// Retrieves the shared borrow for the component of one type attached to the entity.
    /// Returns [`None`][Option::None] if component is not attached to the entity.
    ///
    /// # Examples
    ///
    /// ```
    /// # use toucan_ecs::prelude::*;
    /// # let mut world = World::new();
    /// #[derive(Copy, Clone, Component, Eq, PartialEq, Debug)]
    /// struct Name(&'static str);
    ///
    /// let mut entry = world.create_entry_with(Name("Hello, World"));
    /// let name = entry.get::<Name>().unwrap();
    /// assert_eq!(*name, Name("Hello, World"));
    /// ```
    pub fn get<C>(&'data self) -> Option<&'data C>
    where
        C: Component,
    {
        self.world.get(self.entity)
    }

    /// Retrieves the unique borrow for the component of one type attached to the entity.
    /// Returns [`None`][Option::None] if component is not attached to the entity.
    ///
    /// # Examples
    ///
    /// ```
    /// # use toucan_ecs::prelude::*;
    /// # let mut world = World::new();
    /// #[derive(Copy, Clone, Component, Eq, PartialEq, Debug)]
    /// struct Name(&'static str);
    ///
    /// let mut entry = world.create_entry_with(Name("Hello, World"));
    /// let mut name = entry.get_mut::<Name>().unwrap();
    /// name.0 = "This name was changed";
    /// assert_ne!(*name, Name("Hello, World"));
    /// assert_eq!(*name, Name("This name was changed"));
    /// ```
    pub fn get_mut<C>(&'data mut self) -> Option<&'data mut C>
    where
        C: Component,
    {
        self.world.get_mut(self.entity)
    }
}
