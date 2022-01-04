use crate::component::set::ComponentSet;
use crate::{Component, Entity, Registry};

pub struct Entry<'r> {
    entity: Entity,
    registry: &'r mut Registry,
}

impl<'r> Entry<'r> {
    pub(super) fn new(entity: Entity, registry: &'r mut Registry) -> Self {
        Entry { entity, registry }
    }

    pub fn attach<C>(&mut self, component: C)
    where
        C: Component,
    {
        self.registry.attach(self.entity, component);
    }

    pub fn attach_set<S>(&mut self, set: S)
    where
        S: ComponentSet,
    {
        self.registry.attach_set(self.entity, set)
    }

    pub fn get<C>(&self) -> Option<&C>
    where
        C: Component,
    {
        self.registry.get(self.entity)
    }

    pub fn get_mut<C>(&mut self) -> Option<&mut C>
    where
        C: Component,
    {
        self.registry.get_mut(self.entity)
    }

    pub fn entity(&self) -> Entity {
        self.entity
    }
}
