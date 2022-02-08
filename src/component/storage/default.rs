use std::any::Any;
use std::sync::Mutex;

use slotmap::{SecondaryMap, SlotMap};

use crate::component::{Component, Ref, RefMut};
use crate::entity::Entity;

use super::Storage;

slotmap::new_key_type! {
    struct ComponentKey;
}

#[derive(Default)]
pub struct DefaultStorage<C>
where
    C: Component,
{
    components: SlotMap<ComponentKey, Mutex<C>>,
    mapping: SecondaryMap<Entity, ComponentKey>,
}

impl<C> DefaultStorage<C>
where
    C: Component,
{
    pub fn new() -> Self {
        Self {
            components: SlotMap::with_key(),
            mapping: SecondaryMap::new(),
        }
    }

    pub fn attach(&mut self, entity: Entity, component: C) {
        let component = self.components.insert(Mutex::new(component));
        self.mapping.insert(entity, component);
    }

    pub fn get(&self, entity: Entity) -> Option<Ref<C>> {
        let key = self.mapping.get(entity)?;
        let mutex = self.components.get(*key)?;
        let component = Ref::new(mutex.lock().unwrap());
        Some(component)
    }

    pub fn get_mut(&self, entity: Entity) -> Option<RefMut<C>> {
        let key = self.mapping.get(entity)?;
        let mutex = self.components.get(*key)?;
        let component = RefMut::new(mutex.lock().unwrap());
        Some(component)
    }
}

impl<C> Storage for DefaultStorage<C>
where
    C: Component,
{
    fn remove(&mut self, entity: Entity) {
        let component = self.mapping.remove(entity);
        if let Some(component) = component {
            self.components.remove(component);
        }
    }

    fn attached(&self, entity: Entity) -> bool {
        self.mapping.contains_key(entity)
    }

    fn clear(&mut self) {
        self.mapping.clear();
        self.components.clear();
    }

    fn as_any_ref(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}
