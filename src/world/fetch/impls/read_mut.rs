use crate::component::{Component, Storage, StorageImpl};
use crate::error::{FetchError, FetchResult};
#[cfg(feature = "resource")]
use crate::resource::{marker, Resource};
use crate::world::{FetchMut, WorldDataMut};
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

    unsafe fn new(world: WorldDataMut<'data>) -> FetchResult<Self> {
        // SAFETY: must be checked by the caller.
        let storage = world.components().get_storage().ok_or(FetchError)?;
        Ok(Self { storage })
    }

    fn fetch_mut(&mut self, entity: Entity) -> FetchResult<Self::Item> {
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

        unsafe fn new(world: WorldDataMut<'data>) -> FetchResult<Self> {
            // SAFETY: must be checked by the caller.
            let resource = world.resources().get().ok_or(FetchError)?;
            Ok(Self { resource })
        }

        fn fetch_mut(&mut self, _: Entity) -> FetchResult<Self::Item> {
            let resource = marker::Resource::new(self.resource);
            Ok(resource)
        }
    }
}
