use crate::{Component, Entity, Registry};

pub struct EntityBuilder<'registry> {
    pub(super) entity: Entity,
    pub(super) registry: &'registry mut Registry,
}

impl<'registry> EntityBuilder<'registry> {
    pub fn attach<C>(self, component: C) -> Self
    where
        C: Component,
    {
        self.registry.attach(self.entity, component);
        self
    }

    pub fn build(self) -> Entity {
        self.entity
    }
}
