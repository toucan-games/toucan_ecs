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

    pub fn entity(&self) -> Entity {
        self.entity
    }
}
