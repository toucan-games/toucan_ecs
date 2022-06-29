use std::marker::PhantomData;
use std::ops::{Deref, DerefMut};

use as_any::{AsAny, Downcast};

use crate::component::{Component, StorageImpl};
use crate::Entity;

use super::Storage;

#[repr(transparent)]
pub struct StorageHolder<C>
where
    C: Component,
{
    erased: ErasedStorageHolder,
    _ph: PhantomData<C>,
}

impl<C> From<ErasedStorageHolder> for StorageHolder<C>
where
    C: Component,
{
    fn from(erased: ErasedStorageHolder) -> Self {
        Self {
            erased,
            _ph: PhantomData,
        }
    }
}

impl<C> Deref for StorageHolder<C>
where
    C: Component,
{
    type Target = StorageImpl<C>;

    fn deref(&self) -> &Self::Target {
        self.erased.downcast_ref().unwrap()
    }
}

impl<C> DerefMut for StorageHolder<C>
where
    C: Component,
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.erased.downcast_mut().unwrap()
    }
}

#[repr(transparent)]
pub struct ErasedStorageHolder(Box<dyn Holdable>);

impl ErasedStorageHolder {
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
        self.0.as_ref().downcast_ref()
    }

    pub fn downcast_mut<T>(&mut self) -> Option<&mut T>
    where
        T: Storage,
    {
        self.0.as_mut().downcast_mut()
    }
}

impl<C> From<StorageHolder<C>> for ErasedStorageHolder
where
    C: Component,
{
    fn from(holder: StorageHolder<C>) -> Self {
        holder.erased
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

trait Holdable: AsAny + Send + Sync {
    fn remove(&mut self, entity: Entity);

    fn attached(&self, entity: Entity) -> bool;

    fn clear(&mut self);
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
}
