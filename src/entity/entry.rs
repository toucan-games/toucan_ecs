use crate::component::set::ComponentSet;
use crate::{Component, Entity, Ref, RefMut, Registry};

pub struct Entry<'r> {
    entity: Entity,
    registry: &'r mut Registry,
}

impl<'r> Entry<'r> {
    pub(super) fn new(entity: Entity, registry: &'r mut Registry) -> Self {
        Entry { entity, registry }
    }

    pub fn attach_one<C>(&mut self, component: C)
    where
        C: Component,
    {
        self.registry.attach_one(self.entity, component);
    }

    pub fn attach<S>(&mut self, set: S)
    where
        S: ComponentSet,
    {
        self.registry.attach(self.entity, set)
    }

    pub fn get<C>(&self) -> Option<Ref<C>>
    where
        C: Component,
    {
        self.registry.get(self.entity)
    }

    pub fn get_mut<C>(&mut self) -> Option<RefMut<C>>
    where
        C: Component,
    {
        self.registry.get_mut(self.entity)
    }

    pub fn remove_one<C>(&mut self)
    where
        C: Component,
    {
        self.registry.remove_one::<C>(self.entity)
    }

    pub fn remove<S>(&mut self)
    where
        S: ComponentSet,
    {
        self.registry.remove::<S>(self.entity)
    }

    pub fn remove_all(&mut self) {
        self.registry.remove_all(self.entity)
    }

    pub fn attached_one<C>(&self) -> bool
    where
        C: Component,
    {
        self.registry.attached_one::<C>(self.entity)
    }

    pub fn attached<S>(&self) -> bool
    where
        S: ComponentSet,
    {
        self.registry.attached::<S>(self.entity)
    }

    pub fn destroy(self) {
        self.registry.destroy(self.entity)
    }

    pub fn entity(&self) -> Entity {
        self.entity
    }
}
