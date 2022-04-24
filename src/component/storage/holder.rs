use std::any::Any;
use std::marker::PhantomData;
use std::ops::{Deref, DerefMut};

use as_any::Downcast;

use crate::component::{Component, StorageImpl};
use crate::Entity;

use super::Storage;

#[repr(transparent)]
pub struct StorageHolder<C>
where
    C: Component,
{
    raw: RawStorageHolder,
    _ph: PhantomData<C>,
}

impl<C> From<RawStorageHolder> for StorageHolder<C>
where
    C: Component,
{
    fn from(raw: RawStorageHolder) -> Self {
        Self {
            raw,
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
        self.raw.downcast_ref().unwrap()
    }
}

impl<C> DerefMut for StorageHolder<C>
where
    C: Component,
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.raw.downcast_mut().unwrap()
    }
}

#[repr(transparent)]
pub struct RawStorageHolder(Box<dyn Holdable>);

impl RawStorageHolder {
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

impl<C> From<StorageHolder<C>> for RawStorageHolder
where
    C: Component,
{
    fn from(holder: StorageHolder<C>) -> Self {
        holder.raw
    }
}

impl<T> From<T> for RawStorageHolder
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
