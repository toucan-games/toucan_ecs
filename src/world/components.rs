use crate::component::{
    Component, ComponentSet, Registry as ComponentRegistry, RegistryRefs as StorageRefs,
};
use crate::entity::{Entity, Registry as EntityRegistry};
use crate::prelude::{View, ViewMut, ViewOne, ViewOneMut};
#[cfg(feature = "resource")]
use crate::resource::RegistryRefs as ResourceRefs;
use crate::world::query::{Query, QueryMut};
use crate::world::WorldRefs;

/// **Immutable** borrowed type of the [world](crate::world::World) that contains
/// data of its entities and components.
pub struct Components<'data> {
    entities: &'data EntityRegistry,
    components: &'data ComponentRegistry,
}

impl<'data> Components<'data> {
    pub(super) fn new(
        entities: &'data EntityRegistry,
        components: &'data ComponentRegistry,
    ) -> Self {
        Self {
            entities,
            components,
        }
    }

    /// Returns `true` if the world contains the entity.
    /// Returns `false` if provided entity was destroyed in this world.
    ///
    /// # Examples
    ///
    /// ```
    /// # use toucan_ecs::prelude::*;
    /// # let mut world = World::new();
    /// let entity = world.create();
    /// let components = world.components();
    /// assert!(components.contains(entity));
    /// ```
    pub fn contains(&self, entity: Entity) -> bool {
        self.entities.contains(entity)
    }

    /// Returns `true` if the world does not contain any entity.
    ///
    /// # Examples
    ///
    /// ```
    /// # use toucan_ecs::prelude::*;
    /// let world = World::new();
    /// let components = world.components();
    /// assert!(components.is_empty());
    /// ```
    pub fn is_empty(&self) -> bool {
        self.entities.is_empty()
    }

    /// Returns `true` if the entity does not exist or does not contain any data attached to it.
    ///
    /// # Examples
    ///
    /// ```
    /// # use toucan_ecs::prelude::*;
    /// let mut world = World::new();
    /// let entity = world.create();
    /// let components = world.components();
    /// assert!(components.is_entity_empty(entity));
    /// ```
    pub fn is_entity_empty(&self, entity: Entity) -> bool {
        self.components.is_entity_empty(entity)
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
    /// let components = (Name("Hello, World"), ID(42));
    /// let entity = world.create_with(components);
    ///
    /// let components = world.components();
    /// assert!(components.attached::<(ID, Name)>(entity));
    /// ```
    pub fn attached<S>(&self, entity: Entity) -> bool
    where
        S: ComponentSet,
    {
        self.components.attached::<S>(entity)
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
    /// let entity = world.create_with(Name("Hello, World"));
    /// let components = world.components();
    ///
    /// let name = components.get::<Name>(entity).unwrap();
    /// assert_eq!(*name, Name("Hello, World"));
    /// ```
    pub fn get<C>(&self, entity: Entity) -> Option<&C>
    where
        C: Component,
    {
        self.components.get::<C>(entity)
    }

    /// Creates a [view](ViewOne) of the component type.
    ///
    /// This iterator will return [entities](Entity) and their shared borrows
    /// of components. Only entities that has that type of component will be returned.
    ///
    /// More complex views can be constructed with [`view`][crate::world::World::view()]
    /// associated function.
    ///
    /// # Examples
    ///
    /// ```
    /// # use toucan_ecs::prelude::*;
    /// #[derive(Copy, Clone, Component, Debug)]
    /// struct Name(&'static str);
    ///
    /// let world = World::new();
    /// let components = world.components();
    ///
    /// for (_, component) in components.view_one::<Name>() {
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
    /// let components = world.components();
    ///
    /// for (name, id) in components.view::<(Option<&Name>, &ID)>() {
    ///     println!("name: {:?}, id: {:?}", name.as_deref(), *id)
    /// }
    /// ```
    pub fn view<'view, Q>(&'view self) -> View<'view, Q>
    where
        Q: Query<'view>,
    {
        let (entities, mut data) = self.split_refs();
        let entities = entities.iter();
        View::new(entities, &mut data)
    }

    fn split_refs(&self) -> (&EntityRegistry, WorldRefs) {
        let entities = self.entities;
        let refs = WorldRefs {
            storages: StorageRefs::from(self.components),
            #[cfg(feature = "resource")]
            resources: ResourceRefs::empty(),
        };
        (entities, refs)
    }
}

/// **Mutable** borrowed type of the [world](crate::world::World) that contains
/// data of its entities and components.
pub struct ComponentsMut<'data> {
    entities: &'data EntityRegistry,
    components: &'data mut ComponentRegistry,
}

impl<'data> ComponentsMut<'data> {
    pub(super) fn new(
        entities: &'data EntityRegistry,
        components: &'data mut ComponentRegistry,
    ) -> Self {
        Self {
            entities,
            components,
        }
    }

    /// Returns `true` if the world contains the entity.
    /// Returns `false` if provided entity was destroyed in this world.
    ///
    /// # Examples
    ///
    /// ```
    /// # use toucan_ecs::prelude::*;
    /// # let mut world = World::new();
    /// let entity = world.create();
    /// let components = world.components_mut();
    /// assert!(components.contains(entity));
    /// ```
    pub fn contains(&self, entity: Entity) -> bool {
        self.entities.contains(entity)
    }

    /// Returns `true` if the world does not contain any entity.
    ///
    /// # Examples
    ///
    /// ```
    /// # use toucan_ecs::prelude::*;
    /// let mut world = World::new();
    /// let components = world.components_mut();
    /// assert!(components.is_empty());
    /// ```
    pub fn is_empty(&self) -> bool {
        self.entities.is_empty()
    }

    /// Returns `true` if the entity does not exist or does not contain any data attached to it.
    ///
    /// # Examples
    ///
    /// ```
    /// # use toucan_ecs::prelude::*;
    /// let mut world = World::new();
    /// let entity = world.create();
    /// let components = world.components_mut();
    /// assert!(components.is_entity_empty(entity));
    /// ```
    pub fn is_entity_empty(&self, entity: Entity) -> bool {
        self.components.is_entity_empty(entity)
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
    /// let components = (Name("Hello, World"), ID(42));
    /// let entity = world.create_with(components);
    ///
    /// let components = world.components_mut();
    /// assert!(components.attached::<(ID, Name)>(entity));
    /// ```
    pub fn attached<S>(&self, entity: Entity) -> bool
    where
        S: ComponentSet,
    {
        self.components.attached::<S>(entity)
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
    /// let entity = world.create_with(Name("Hello, World"));
    /// let components = world.components_mut();
    ///
    /// let name = components.get::<Name>(entity).unwrap();
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
    /// let entity = world.create_with(Name("Hello, World"));
    /// let mut components = world.components_mut();
    ///
    /// let mut name = components.get_mut::<Name>(entity).unwrap();
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

    /// Creates a [view](ViewOne) of the component type.
    ///
    /// This iterator will return [entities](Entity) and their shared borrows
    /// of components. Only entities that has that type of component will be returned.
    ///
    /// More complex views can be constructed with [`view`][crate::world::World::view()]
    /// associated function.
    ///
    /// # Examples
    ///
    /// ```
    /// # use toucan_ecs::prelude::*;
    /// #[derive(Copy, Clone, Component, Debug)]
    /// struct Name(&'static str);
    ///
    /// let mut world = World::new();
    /// let components = world.components_mut();
    ///
    /// for (_, component) in components.view_one::<Name>() {
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
    /// let mut components = world.components_mut();
    ///
    /// for (_, component) in components.view_one_mut::<Name>() {
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
    /// let mut world = World::new();
    /// let components = world.components_mut();
    ///
    /// for (name, id) in components.view::<(Option<&Name>, &ID)>() {
    ///     println!("name: {:?}, id: {:?}", name.as_deref(), *id)
    /// }
    /// ```
    pub fn view<'view, Q>(&'view self) -> View<'view, Q>
    where
        Q: Query<'view>,
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
    /// let mut components = world.components_mut();
    ///
    /// // immutable ID reference and mutable one of the same type are illegal
    /// for (id, mut mut_id) in components.view_mut::<(&ID, &mut ID)>() {
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
    pub fn view_mut<'view, Q>(&'view mut self) -> ViewMut<'view, Q>
    where
        Q: QueryMut<'view>,
    {
        let (entities, mut data) = self.split_refs_mut();
        let entities = entities.iter();
        ViewMut::new(entities, &mut data)
    }

    fn split_refs(&self) -> (&EntityRegistry, WorldRefs) {
        let entities = self.entities;
        let refs = WorldRefs {
            storages: StorageRefs::from(&*self.components),
            #[cfg(feature = "resource")]
            resources: ResourceRefs::empty(),
        };
        (entities, refs)
    }

    fn split_refs_mut(&mut self) -> (&EntityRegistry, WorldRefs) {
        let entities = self.entities;
        let refs = WorldRefs {
            storages: StorageRefs::from(&mut *self.components),
            #[cfg(feature = "resource")]
            resources: ResourceRefs::empty(),
        };
        (entities, refs)
    }
}
