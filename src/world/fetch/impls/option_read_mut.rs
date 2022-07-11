use crate::component::{Component, StorageHolder};
use crate::error::FetchResult;
#[cfg(feature = "resource")]
use crate::resource::{marker, Resource};
use crate::world::{FetchMut, WorldDataMut};
use crate::Entity;

#[repr(transparent)]
pub struct FetchOptionReadMut<'data, C>
where
    C: Component,
{
    storage: Option<StorageHolder<'data, C>>,
}

impl<'data, C> FetchMut<'data> for FetchOptionReadMut<'data, C>
where
    C: Component,
{
    type Item = Option<&'data C>;

    // noinspection DuplicatedCode
    unsafe fn new(world: WorldDataMut<'data>) -> FetchResult<Self> {
        // SAFETY: must be checked by the caller.
        let storage = world.components().get_storage();
        Ok(Self { storage })
    }

    fn entities(&self) -> Option<Box<dyn ExactSizeIterator<Item = Entity> + Send + Sync + 'data>> {
        None
    }

    fn fetch_mut(&'data mut self, entity: Entity) -> FetchResult<Self::Item> {
        let item = self.storage.and_then(|storage| storage.get(entity));
        Ok(item)
    }
}

cfg_resource! {
    #[repr(transparent)]
    pub struct FetchResourceOptionReadMut<'data, R>
    where
        R: Resource,
    {
        resource: Option<&'data R>,
    }

    impl<'data, R> FetchMut<'data> for FetchResourceOptionReadMut<'data, R>
    where
        R: Resource,
    {
        type Item = Option<marker::Resource<'data, R>>;

        unsafe fn new(world: WorldDataMut<'data>) -> FetchResult<Self> {
            // SAFETY: must be checked by the caller.
            let resource = world.resources().get();
            Ok(Self { resource })
        }

        fn entities(&self) -> Option<Box<dyn ExactSizeIterator<Item=Entity> + Send + Sync + 'data>> {
            None
        }

        fn fetch_mut(&mut self, _: Entity) -> FetchResult<Self::Item> {
            let resource = self.resource.map(marker::Resource::new);
            Ok(resource)
        }
    }
}
