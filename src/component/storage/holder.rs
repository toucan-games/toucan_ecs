use std::any::Any;

use as_any::{AsAny, Downcast};

use crate::Entity;

use super::Storage;

pub struct StorageHolder(Box<dyn Holdable>);

impl StorageHolder {
    pub fn remove(&mut self, entity: Entity) {
        self.0.remove(entity)
    }

    pub fn attached(&self, entity: Entity) -> bool {
        self.0.attached(entity)
    }

    pub fn clear(&mut self) {
        self.0.clear()
    }

    pub fn downcast_ref<T>(&self) -> Option<&T>
    where
        T: Storage,
    {
        self.0.as_ref().as_any().downcast_ref()
    }

    pub fn downcast_mut<T>(&mut self) -> Option<&mut T>
    where
        T: Storage,
    {
        self.0.as_mut().as_any_mut().downcast_mut()
    }
}

impl<T> From<T> for StorageHolder
where
    T: Holdable,
{
    fn from(storage: T) -> Self {
        Self(Box::new(storage))
    }
}

trait Holdable: 'static {
    fn remove(&mut self, entity: Entity);

    fn attached(&self, entity: Entity) -> bool;

    fn clear(&mut self);

    fn as_any(&self) -> &dyn Any;

    fn as_any_mut(&mut self) -> &mut dyn Any;
}

impl<T> Holdable for T
where
    T: Storage,
{
    fn remove(&mut self, entity: Entity) {
        self.remove(entity)
    }

    fn attached(&self, entity: Entity) -> bool {
        self.attached(entity)
    }

    fn clear(&mut self) {
        self.clear()
    }

    fn as_any(&self) -> &dyn Any {
        self.downcast_ref::<T>().unwrap()
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self.downcast_mut::<T>().unwrap()
    }
}
