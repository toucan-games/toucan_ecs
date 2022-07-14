use crate::component::storage::Storage;
use crate::component::Component;
use crate::entity::Entity;
use crate::error::FetchResult;
#[cfg(feature = "resource")]
use crate::resource::{marker, Resource};
use crate::world::WorldDataMut;

#[repr(transparent)]
pub struct FetchOptionWrite<'data, C>
where
    C: Component,
{
    storage: Option<&'data mut C::Storage>,
}

impl<'data, C> FetchOptionWrite<'data, C>
where
    C: Component,
{
    pub unsafe fn new(world: WorldDataMut<'data>) -> Self {
        let storage = world.components_mut().get_storage_mut::<C>();
        Self { storage }
    }

    pub fn fetch_mut(&'data mut self, entity: Entity) -> FetchResult<Option<&'data mut C>> {
        let storage = self.storage.as_mut();
        let item = storage.and_then(|storage| storage.get_mut(entity));
        Ok(item)
    }
}

cfg_resource! {
    #[repr(transparent)]
    pub struct FetchResourceOptionWrite<'data, R>
    where
        R: Resource,
    {
        resource: Option<&'data mut R>,
    }

    impl<'data, R> FetchResourceOptionWrite<'data, R>
    where
        R: Resource,
    {
        pub unsafe fn new(world: WorldDataMut<'data>) -> Self {
            let resource = world.resources_mut().get_mut();
            Self { resource }
        }

        pub fn fetch_mut(&'data mut self) -> Option<marker::ResourceMut<'data, R>> {
            self.resource.as_mut().map(|it| marker::ResourceMut::new(*it))
        }
    }
}
