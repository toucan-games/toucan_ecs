use slotmap::{SecondaryMap, SlotMap};

use crate::{Component, Entity};

slotmap::new_key_type! {
    struct ComponentKey;
}

pub struct ComponentPool<C>
where
    C: Component,
{
    components: SlotMap<ComponentKey, C>,
    mapping: SecondaryMap<Entity, ComponentKey>,
}

impl<C> ComponentPool<C>
where
    C: Component,
{
    pub fn new() -> Self {
        Self {
            components: SlotMap::with_key(),
            mapping: SecondaryMap::new(),
        }
    }

    pub fn save(&mut self, entity: Entity, component: C) {
        let component = self.components.insert(component);
        self.mapping.insert(entity, component);
    }

    pub fn get(&self, entity: Entity) -> Option<&C> {
        let component = self.mapping.get(entity)?;
        self.components.get(*component)
    }

    pub fn get_mut(&mut self, entity: Entity) -> Option<&mut C> {
        let component = self.mapping.get(entity)?;
        self.components.get_mut(*component)
    }

    pub fn remove(&mut self, entity: Entity) {
        let component = self.mapping.remove(entity);
        if let Some(component) = component {
            self.components.remove(component);
        }
    }

    pub fn contains(&self, entity: Entity) -> bool {
        self.mapping.contains_key(entity)
    }
}
