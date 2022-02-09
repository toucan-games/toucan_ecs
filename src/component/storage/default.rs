use std::any::Any;
use std::sync::Mutex;

use slotmap::SecondaryMap;

use crate::component::{Component, Ref, RefMut};
use crate::entity::Entity;

use super::Storage;

#[derive(Default)]
#[repr(transparent)]
pub struct DefaultStorage<C>
where
    C: Component,
{
    components: SecondaryMap<Entity, Mutex<C>>,
}

impl<C> DefaultStorage<C>
where
    C: Component,
{
    pub fn new() -> Self {
        Self {
            components: SecondaryMap::new(),
        }
    }

    pub fn attach(&mut self, entity: Entity, component: C) {
        self.components.insert(entity, Mutex::new(component));
    }

    pub fn get(&self, entity: Entity) -> Option<Ref<C>> {
        let mutex = self.components.get(entity)?;
        let component = Ref::new(mutex.lock().unwrap());
        Some(component)
    }

    pub fn get_mut(&self, entity: Entity) -> Option<RefMut<C>> {
        let mutex = self.components.get(entity)?;
        let component = RefMut::new(mutex.lock().unwrap());
        Some(component)
    }
}

impl<C> Storage for DefaultStorage<C>
where
    C: Component,
{
    fn remove(&mut self, entity: Entity) {
        self.components.remove(entity);
    }

    fn attached(&self, entity: Entity) -> bool {
        self.components.contains_key(entity)
    }

    fn clear(&mut self) {
        self.components.clear();
    }

    fn as_any_ref(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}
