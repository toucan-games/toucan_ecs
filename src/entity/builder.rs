use std::collections::HashMap;
use std::hash::BuildHasherDefault;

use crate::component::{Component, ComponentTypeId, Registry};
use crate::hash::TypeIdHasher;
use crate::Entity;

/// Allows for building the new entity with **builder** pattern.
///
/// This struct could be used to create new entity lazily
/// based on some conditions which can change at runtime.
///
/// Note that this struct is *lazy* and does nothing unless being built.
/// Entity will be actually created on [`build`][build] function call.
///
/// [build]: EntityBuilder::build()
#[must_use = "Please call .build() on this to finish building the new entity"]
pub struct EntityBuilder<'data> {
    registry: &'data mut Registry,
    data: HashMap<ComponentTypeId, ErasedComponentHolder, BuildHasherDefault<TypeIdHasher>>,
}

impl<'data> EntityBuilder<'data> {
    pub(crate) fn new(registry: &'data mut Registry) -> Self {
        Self {
            registry,
            data: HashMap::default(),
        }
    }

    /// Saves provided component in temporary storage to attach it later
    /// on [`build`][build] function call.
    ///
    /// Repeated function call with the same type of component
    /// will replace previous value with new one.
    ///
    /// [build]: EntityBuilder::build()
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
        let entity = self.registry.create();
        for holder in self.data.into_values() {
            holder.attach(entity, self.registry);
        }
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
    pub fn attach(&self, entity: Entity, registry: &mut Registry) {
        self.0.attach(entity, registry)
    }
}

trait Holdable: Send + Sync + 'static {
    fn attach(&self, entity: Entity, registry: &mut Registry);
}

impl<C> Holdable for C
where
    C: Component,
{
    fn attach(&self, entity: Entity, registry: &mut Registry) {
        let component = *self;
        registry.attach_one(entity, component)
    }
}
