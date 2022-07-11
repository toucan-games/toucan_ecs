use crate::component::{Component, StorageHolder};
#[cfg(feature = "resource")]
use crate::resource::{marker, Resource};
use crate::world::WorldData;
use crate::Entity;

#[repr(transparent)]
pub struct FetchOptionRead<'data, C>
where
    C: Component,
{
    storage: Option<StorageHolder<'data, C>>,
}

impl<'data, C> FetchOptionRead<'data, C>
where
    C: Component,
{
    pub fn new(data: WorldData<'data>) -> Self {
        let storage = data.components().get_storage();
        Self { storage }
    }

    pub fn fetch(&self, entity: Entity) -> Option<&'data C> {
        self.storage.and_then(|storage| storage.get(entity))
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

    impl<'data, R> FetchResourceOptionRead<'data, R>
    where
        R: Resource,
    {
        pub fn new(world: WorldData<'data>) -> Self {
            let resource = world.resources().get();
            Self { resource }
        }

        pub fn fetch(&self) -> Option<marker::Resource<'data, R>> {
            self.resource.map(marker::Resource::new)
        }
    }
}
