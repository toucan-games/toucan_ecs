use crate::component::{Component, StorageHolderMut};
use crate::error::FetchResult;
#[cfg(feature = "resource")]
use crate::resource::{marker, Resource};
use crate::world::{FetchMut, WorldDataMut};
use crate::Entity;

#[repr(transparent)]
pub struct FetchOptionWriteMut<'data, C>
where
    C: Component,
{
    storage: Option<StorageHolderMut<'data, C>>,
}

impl<'data, C> FetchMut<'data> for FetchOptionWriteMut<'data, C>
where
    C: Component,
{
    type Item = Option<&'data mut C>;

    unsafe fn new(world: WorldDataMut<'data>) -> FetchResult<Self> {
        // SAFETY: must be checked by the caller.
        let storage = world.components_mut().get_storage_mut();
        Ok(Self { storage })
    }

    fn entities(&self) -> Option<Box<dyn ExactSizeIterator<Item = Entity> + Send + Sync + 'data>> {
        None
    }

    fn fetch_mut(&'data mut self, entity: Entity) -> FetchResult<Self::Item> {
        let storage = self.storage.as_mut();
        let item = storage.and_then(|storage| storage.get_mut(entity));
        Ok(item)
    }
}

cfg_resource! {
    #[repr(transparent)]
    pub struct FetchResourceOptionWriteMut<'data, R>
    where
        R: Resource,
    {
        resource: Option<&'data mut R>,
    }

    impl<'data, R> FetchMut<'data> for FetchResourceOptionWriteMut<'data, R>
    where
        R: Resource,
    {
        type Item = Option<marker::ResourceMut<'data, R>>;

        unsafe fn new(world: WorldDataMut<'data>) -> FetchResult<Self> {
            // SAFETY: must be checked by the caller.
            let resource = world.resources_mut().get_mut();
            Ok(Self { resource })
        }

        fn entities(&self) -> Option<Box<dyn ExactSizeIterator<Item=Entity> + Send + Sync + 'data>> {
            None
        }

        fn fetch_mut(&'data mut self, _: Entity) -> FetchResult<Self::Item> {
            let resource = self.resource.as_mut().map(|it| marker::ResourceMut::new(*it));
            Ok(resource)
        }
    }
}
