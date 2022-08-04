use std::collections::HashMap;
use std::hash::BuildHasherDefault;

use crate::component::{Component, ComponentTypeId};
use crate::entity::Entity;
use crate::hash::TypeIdHasher;
use crate::world::World;

/// Allows for building the new entity with **builder** pattern.
///
/// This struct could be used to create new entity lazily
/// based on some conditions which can change at runtime.
///
/// Note that this struct is *lazy* and does nothing unless being built.
/// Entity will be actually created on [`build`] function call.
///
/// [`build`]: EntityBuilder::build()
#[must_use = "Please call .build() to create a new entity"]
pub struct EntityBuilder<'data> {
    world: &'data mut World,
    data: HashMap<ComponentTypeId, ErasedComponentHolder, BuildHasherDefault<TypeIdHasher>>,
}

impl<'data> EntityBuilder<'data> {
    pub(crate) fn new(world: &'data mut World) -> Self {
        let data = HashMap::default();
        Self { world, data }
    }

    /// Saves provided component in temporary storage to attach it later
    /// on [`build`] function call.
    ///
    /// Repeated function call with the same type of component
    /// will replace previous value with new one.
    ///
    /// [`build`]: EntityBuilder::build()
    pub fn with<C>(mut self, component: C) -> Self
    where
        C: Component,
    {
        let type_id = ComponentTypeId::of::<C>();
        self.data.insert(type_id, component.into());
        self
    }

    /// Finalizes the builder, attaches all saved components
    /// and returns new [entity](crate::entity::Entity) handle.
    pub fn build(self) -> Entity {
        let entity = self.world.create();
        let world = self.world;
        self.data
            .into_values()
            .for_each(|holder| holder.attach(entity, world));
        entity
    }
}

#[repr(transparent)]
pub struct ErasedComponentHolder(Box<dyn Holdable>);

impl<C> From<C> for ErasedComponentHolder
where
    C: Component,
{
    fn from(component: C) -> Self {
        Self(Box::new(component))
    }
}

impl ErasedComponentHolder {
    pub fn attach(self, entity: Entity, world: &mut World) {
        self.0.attach(entity, world)
    }
}

trait Holdable: Send + Sync + 'static {
    fn attach(self: Box<Self>, entity: Entity, world: &mut World);
}

impl<C> Holdable for C
where
    C: Component,
{
    fn attach(self: Box<Self>, entity: Entity, world: &mut World) {
        let component = *self;
        world.attach(entity, component)
    }
}
