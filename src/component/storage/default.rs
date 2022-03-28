use atomic_refcell::{AtomicRef, AtomicRefCell, AtomicRefMut};
use slotmap::SecondaryMap;

use crate::component::Component;
use crate::entity::Entity;

use super::Storage;

#[derive(Default)]
#[repr(transparent)]
pub struct DefaultStorage<C>
where
    C: Component,
{
    components: SecondaryMap<Entity, AtomicRefCell<C>>,
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
        self.components
            .insert(entity, AtomicRefCell::new(component));
    }

    pub fn get(&self, entity: Entity) -> Option<AtomicRef<C>> {
        self.components.get(entity).map(|it| it.borrow())
    }

    pub fn get_im_mut(&self, entity: Entity) -> Option<AtomicRefMut<C>> {
        self.components.get(entity).map(|it| it.borrow_mut())
    }

    pub fn get_mut(&mut self, entity: Entity) -> Option<&mut C> {
        self.components.get_mut(entity).map(|it| it.get_mut())
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
}
