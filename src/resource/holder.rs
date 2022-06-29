use std::marker::PhantomData;
use std::ops::{Deref, DerefMut};

use as_any::{AsAny, Downcast};

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
        self.erased.downcast_mut().expect("downcast error")
    }
}

impl<R> Deref for ResourceHolder<R>
where
    R: Resource,
{
    type Target = R;

    fn deref(&self) -> &Self::Target {
        self.erased.downcast_ref().expect("downcast error")
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
        self.0.as_ref().downcast_ref()
    }

    pub fn downcast_mut<R>(&mut self) -> Option<&mut R>
    where
        R: Resource,
    {
        self.0.as_mut().downcast_mut()
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

impl<R> From<(R, )> for ErasedResourceHolder
where
    R: Resource,
{
    fn from(resource: (R, )) -> Self {
        Self(Box::new(resource.0))
    }
}

trait Holdable: AsAny + Send + Sync {}

impl<R> Holdable for R where R: Resource {}
