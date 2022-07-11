use crate::component::{Component, StorageHolder};
use crate::error::FetchResult;
#[cfg(feature = "resource")]
use crate::resource::{marker, Resource};
use crate::world::{Fetch, WorldData};
use crate::Entity;

#[repr(transparent)]
pub struct FetchOptionRead<'data, C>
where
    C: Component,
{
    storage: Option<StorageHolder<'data, C>>,
}

impl<'data, C> Fetch<'data> for FetchOptionRead<'data, C>
where
    C: Component,
{
    type Item = Option<&'data C>;

    // noinspection DuplicatedCode
    fn new(world: WorldData<'data>) -> FetchResult<Self> {
        let storage = world.components().get_storage();
        Ok(Self { storage })
    }

    fn entities(&self) -> Option<Box<dyn ExactSizeIterator<Item = Entity> + Send + Sync + 'data>> {
        None
    }

    fn fetch(&self, entity: Entity) -> FetchResult<Self::Item> {
        let item = self.storage.and_then(|storage| storage.get(entity));
        Ok(item)
    }
}

cfg_resource! {
    #[repr(transparent)]
    pub struct FetchResourceOptionRead<'data, R>
    where
        R: Resource,
    {
        resource: Option<&'data R>,
    }

    impl<'data, R> Fetch<'data> for FetchResourceOptionRead<'data, R>
    where
        R: Resource,
    {
        type Item = Option<marker::Resource<'data, R>>;

        fn new(world: WorldData<'data>) -> FetchResult<Self> {
            let resource = world.resources().get();
            Ok(Self { resource })
        }

        fn entities(&self) -> Option<Box<dyn ExactSizeIterator<Item=Entity> + Send + Sync + 'data>> {
            None
        }

        fn fetch(&self, _: Entity) -> FetchResult<Self::Item> {
            let resource = self.resource.map(marker::Resource::new);
            Ok(resource)
        }
    }
}
