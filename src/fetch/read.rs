use crate::component::storage::Storage;
use crate::component::Component;
use crate::entity::Entity;
#[cfg(feature = "resource")]
use crate::resource::{marker, Resource};
use crate::world::WorldData;

#[repr(transparent)]
pub struct FetchRead<'data, C>
where
    C: Component,
{
    storage: &'data C::Storage,
}

impl<'data, C> FetchRead<'data, C>
where
    C: Component,
{
    pub fn new(data: WorldData<'data>) -> Option<Self> {
        let storage = data.components().get_storage::<C>()?;
        Some(Self { storage })
    }

    pub fn entities(&self) -> Option<impl ExactSizeIterator<Item = Entity> + Send + Sync + 'data> {
        let iter = self.storage.iter();
        let iter = iter.map(|(entity, _)| entity);
        Some(iter)
    }

    pub fn fetch(&self, entity: Entity) -> Option<&'data C> {
        self.storage.get(entity)
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

    impl<'data, R> FetchResourceRead<'data, R>
    where
        R: Resource,
    {
        pub fn new(data: WorldData<'data>) -> Option<Self> {
            let resource = data.resources().get()?;
            Some(Self { resource })
        }

        pub fn fetch(&self) -> marker::Resource<'data, R> {
            marker::Resource::new(self.resource)
        }
    }
}
