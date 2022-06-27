use crate::component::{Component, Storage, StorageImpl};
#[cfg(feature = "resource")]
use crate::resource::{marker, Resource};
use crate::world::{FetchError, FetchMut, WorldDataMut};
use crate::Entity;

#[repr(transparent)]
pub struct FetchReadMut<'data, C>
where
    C: Component,
{
    storage: &'data StorageImpl<C>,
}

impl<'data, C> FetchMut<'data> for FetchReadMut<'data, C>
where
    C: Component,
{
    type Item = &'data C;

    unsafe fn new(world: WorldDataMut<'data>) -> Result<Self, FetchError> {
        // SAFETY: must be checked by the caller.
        let storage = world.components().get_storage().ok_or(FetchError)?;
        Ok(Self { storage })
    }

    fn fetch_mut(&mut self, entity: Entity) -> Result<Self::Item, FetchError> {
        self.storage.get(entity).ok_or(FetchError)
    }
}

cfg_resource! {
    #[repr(transparent)]
    pub struct FetchResourceReadMut<'data, R>
    where
        R: Resource,
    {
        resource: &'data R,
    }

    impl<'data, R> FetchMut<'data> for FetchResourceReadMut<'data, R>
    where
        R: Resource,
    {
        type Item = marker::Resource<'data, R>;

        unsafe fn new(world: WorldDataMut<'data>) -> Result<Self, FetchError> {
            // SAFETY: must be checked by the caller.
            let resource = world.resources().get().ok_or(FetchError)?;
            Ok(Self { resource })
        }

        fn fetch_mut(&mut self, _: Entity) -> Result<Self::Item, FetchError> {
            let resource = marker::Resource::new(self.resource);
            Ok(resource)
        }
    }
}
