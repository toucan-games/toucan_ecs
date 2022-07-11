use crate::component::{Component, StorageHolderMut};
#[cfg(feature = "resource")]
use crate::resource::{marker, Resource};
use crate::world::WorldDataMut;
use crate::Entity;

#[repr(transparent)]
pub struct FetchWrite<'data, C>
where
    C: Component,
{
    storage: StorageHolderMut<'data, C>,
}

impl<'data, C> FetchWrite<'data, C>
where
    C: Component,
{
    pub unsafe fn new(world: WorldDataMut<'data>) -> Option<Self> {
        let storage = world.components_mut().get_storage_mut()?;
        Some(Self { storage })
    }

    pub fn entities(&self) -> Option<impl ExactSizeIterator<Item = Entity> + Send + Sync + 'data> {
        let iter = self.storage.iter().map(|(entity, _)| entity);
        Some(iter)
    }

    pub fn fetch_mut(&'data mut self, entity: Entity) -> Option<&'data mut C> {
        self.storage.get_mut(entity)
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

    impl<'data, R> FetchResourceWrite<'data, R>
    where
        R: Resource,
    {
        pub unsafe fn new(data: WorldDataMut<'data>) -> Option<Self> {
            let resource = data.resources_mut().get_mut()?;
            Some(Self { resource })
        }

        pub fn fetch_mut(&'data mut self) -> marker::ResourceMut<'data, R> {
            marker::ResourceMut::new(self.resource)
        }
    }
}
