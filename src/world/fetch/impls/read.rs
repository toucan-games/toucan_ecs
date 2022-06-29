use crate::component::{Component, StorageHolder};
use crate::error::{FetchError, FetchResult};
#[cfg(feature = "resource")]
use crate::resource::{marker, Resource};
use crate::world::{Fetch, WorldData};
use crate::Entity;

#[repr(transparent)]
pub struct FetchRead<'data, C>
where
    C: Component,
{
    storage: StorageHolder<'data, C>,
}

impl<'data, C> Fetch<'data> for FetchRead<'data, C>
where
    C: Component,
{
    type Item = &'data C;

    // noinspection DuplicatedCode
    fn new(world: WorldData<'data>) -> FetchResult<Self> {
        let storage = world.components().get_storage().ok_or(FetchError)?;
        Ok(Self { storage })
    }

    fn fetch(&self, entity: Entity) -> FetchResult<Self::Item> {
        self.storage.get(entity).ok_or(FetchError)
    }
}

cfg_resource! {
    #[repr(transparent)]
    pub struct FetchResourceRead<'data, R>
    where
        R: Resource,
    {
        resource: &'data R,
    }

    impl<'data, R> Fetch<'data> for FetchResourceRead<'data, R>
    where
        R: Resource,
    {
        type Item = marker::Resource<'data, R>;

        fn new(world: WorldData<'data>) -> FetchResult<Self> {
            let resource = world.resources().get().ok_or(FetchError)?;
            Ok(Self { resource })
        }

        fn fetch(&self, _: Entity) -> FetchResult<Self::Item> {
            let resource = marker::Resource::new(self.resource);
            Ok(resource)
        }
    }
}
