use std::any::Any;
use std::marker::PhantomData;
use std::ops::{Deref, DerefMut};

use as_any::Downcast;

use super::Resource;

#[repr(transparent)]
pub struct ResourceHolder<R>
where
    R: Resource,
{
    erased: ErasedResourceHolder,
    _ph: PhantomData<R>,
}

impl<R> DerefMut for ResourceHolder<R>
where
    R: Resource,
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.erased.downcast_mut().unwrap()
    }
}

impl<R> Deref for ResourceHolder<R>
where
    R: Resource,
{
    type Target = R;

    fn deref(&self) -> &Self::Target {
        self.erased.downcast_ref().unwrap()
    }
}

impl<R> From<ErasedResourceHolder> for ResourceHolder<R>
where
    R: Resource,
{
    fn from(erased: ErasedResourceHolder) -> Self {
        Self {
            erased,
            _ph: PhantomData,
        }
    }
}

#[repr(transparent)]
pub struct ErasedResourceHolder(Box<dyn Holdable>);

impl ErasedResourceHolder {
    pub fn downcast_ref<R>(&self) -> Option<&R>
    where
        R: Resource,
    {
        self.0.as_ref().as_any().downcast_ref()
    }

    pub fn downcast_mut<R>(&mut self) -> Option<&mut R>
    where
        R: Resource,
    {
        self.0.as_mut().as_any_mut().downcast_mut()
    }
}

impl<R> From<ResourceHolder<R>> for ErasedResourceHolder
where
    R: Resource,
{
    fn from(holder: ResourceHolder<R>) -> Self {
        holder.erased
    }
}

impl<T> From<(T,)> for ErasedResourceHolder
where
    T: Resource,
{
    fn from(resource: (T,)) -> Self {
        Self(Box::new(resource.0))
    }
}

trait Holdable: 'static {
    fn as_any(&self) -> &dyn Any;

    fn as_any_mut(&mut self) -> &mut dyn Any;
}

impl<R> Holdable for R
where
    R: Resource,
{
    fn as_any(&self) -> &dyn Any {
        self.downcast_ref::<R>().unwrap()
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self.downcast_mut::<R>().unwrap()
    }
}
