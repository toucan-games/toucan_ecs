use as_any::{AsAny, Downcast};

use super::Resource;

#[repr(transparent)]
pub struct ErasedResourceHolder(Box<dyn Holdable>);

impl ErasedResourceHolder {
    pub fn as_resource_ref<R>(&self) -> Option<&R>
    where
        R: Resource,
    {
        self.0.as_ref().downcast_ref()
    }

    pub fn as_resource_mut<R>(&mut self) -> Option<&mut R>
    where
        R: Resource,
    {
        self.0.as_mut().downcast_mut()
    }
}

impl<R> From<(R,)> for ErasedResourceHolder
where
    R: Resource,
{
    fn from(resource: (R,)) -> Self {
        Self(Box::new(resource.0))
    }
}

trait Holdable: AsAny + Send + Sync {}

impl<R> Holdable for R where R: Resource {}
