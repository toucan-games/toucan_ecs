use as_any::{AsAny, Downcast};

use crate::component::storage::Storage;
use crate::component::Component;
use crate::entity::Entity;

#[repr(transparent)]
pub struct ErasedStorageHolder(Box<dyn Holdable>);

impl ErasedStorageHolder {
    pub fn attached(&self, entity: Entity) -> bool {
        self.0.attached(entity)
    }

    pub fn remove(&mut self, entity: Entity) {
        self.0.remove(entity)
    }

    pub fn clear(&mut self) {
        self.0.clear()
    }

    pub fn as_storage_ref<C, S>(&self) -> &S
    where
        S: Storage<Item = C>,
        C: Component,
    {
        self.0.as_ref().downcast_ref().expect("downcast error")
    }

    pub fn as_storage_mut<C, S>(&mut self) -> &mut S
    where
        S: Storage<Item = C>,
        C: Component,
    {
        self.0.as_mut().downcast_mut().expect("downcast error")
    }
}

impl<T> From<T> for ErasedStorageHolder
where
    T: Storage,
{
    fn from(storage: T) -> Self {
        Self(Box::new(storage))
    }
}

pub trait Holdable: AsAny + Send + Sync {
    fn attached(&self, entity: Entity) -> bool;

    fn remove(&mut self, entity: Entity);

    fn clear(&mut self);
}

impl<T> Holdable for T
where
    T: Storage,
{
    fn attached(&self, entity: Entity) -> bool {
        self.attached(entity)
    }

    fn remove(&mut self, entity: Entity) {
        self.remove(entity)
    }

    fn clear(&mut self) {
        self.clear()
    }
}
