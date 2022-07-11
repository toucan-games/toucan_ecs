use crate::component::{Component, StorageHolderMut};
use crate::error::{FetchError, FetchResult};
#[cfg(feature = "resource")]
use crate::resource::{marker, Resource};
use crate::system::foreach::fetch::Fetch;
use crate::world::WorldDataMut;
use crate::Entity;

#[repr(transparent)]
pub struct FetchWrite<'data, C>
where
    C: Component,
{
    storage: StorageHolderMut<'data, C>,
}

impl<'data, C> Fetch<'data> for FetchWrite<'data, C>
where
    C: Component,
{
    type Item = &'data mut C;

    // noinspection DuplicatedCode
    unsafe fn new(world: WorldDataMut<'data>) -> FetchResult<Self> {
        // SAFETY: must be checked by the caller.
        let storage = world.components_mut().get_storage_mut().ok_or(FetchError)?;
        Ok(Self { storage })
    }

    // noinspection DuplicatedCode
    fn entities(&self) -> Option<Box<dyn ExactSizeIterator<Item = Entity> + Send + Sync + 'data>> {
        let iter = self.storage.iter().map(|(entity, _)| entity);
        Some(Box::new(iter))
    }

    fn fetch(&'data mut self, entity: Entity) -> FetchResult<Self::Item> {
        self.storage.get_mut(entity).ok_or(FetchError)
    }
}

cfg_resource! {
    #[repr(transparent)]
    pub struct FetchResourceWrite<'data, R>
    where
        R: Resource,
    {
        resource: &'data mut R,
    }

    impl<'data, R> Fetch<'data> for FetchResourceWrite<'data, R>
    where
        R: Resource,
    {
        type Item = marker::ResourceMut<'data, R>;

        unsafe fn new(world: WorldDataMut<'data>) -> FetchResult<Self> {
            // SAFETY: must be checked by the caller.
            let resource = world.resources_mut().get_mut().ok_or(FetchError)?;
            Ok(Self { resource })
        }

        fn entities(&self) -> Option<Box<dyn ExactSizeIterator<Item=Entity> + Send + Sync + 'data>> {
            None
        }

        fn fetch(&'data mut self, _: Entity) -> FetchResult<Self::Item> {
            let resource = marker::ResourceMut::new(self.resource);
            Ok(resource)
        }
    }
}
