use crate::component::{Component, Storage, StorageImpl};
#[cfg(feature = "resource")]
use crate::resource::{marker, Resource};
use crate::world::{FetchError, FetchMut, WorldDataMut};
use crate::Entity;

#[repr(transparent)]
pub struct FetchWriteMut<'data, C>
where
    C: Component,
{
    storage: &'data mut StorageImpl<C>,
}

impl<'data, C> FetchMut<'data> for FetchWriteMut<'data, C>
where
    C: Component,
{
    type Item = &'data mut C;

    unsafe fn new(world: WorldDataMut<'data>) -> Result<Self, FetchError> {
        // SAFETY: must be checked by the caller.
        let storage = world.components_mut().get_storage_mut().ok_or(FetchError)?;
        Ok(Self { storage })
    }

    fn fetch_mut(&'data mut self, entity: Entity) -> Result<Self::Item, FetchError> {
        self.storage.get_mut(entity).ok_or(FetchError)
    }
}

cfg_resource! {
    #[repr(transparent)]
    pub struct FetchResourceWriteMut<'data, R>
    where
        R: Resource,
    {
        resource: &'data mut R,
    }

    impl<'data, R> FetchMut<'data> for FetchResourceWriteMut<'data, R>
    where
        R: Resource,
    {
        type Item = marker::ResourceMut<'data, R>;

        unsafe fn new(world: WorldDataMut<'data>) -> Result<Self, FetchError> {
            // SAFETY: must be checked by the caller.
            let resource = world.resources_mut().get_mut().ok_or(FetchError)?;
            Ok(Self { resource })
        }

        fn fetch_mut(&'data mut self, _: Entity) -> Result<Self::Item, FetchError> {
            let resource = marker::ResourceMut::new(self.resource);
            Ok(resource)
        }
    }
}
