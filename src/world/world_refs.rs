use crate::component::{Component, RegistryRefs as StorageRefs};
#[cfg(feature = "resource")]
use crate::resource::{RegistryRefs as ResourceRefs, Resource};

pub struct WorldRefs<'world> {
    pub(super) storages: StorageRefs<'world>,
    #[cfg(feature = "resource")]
    pub(super) resources: ResourceRefs<'world>,
}

impl<'world> WorldRefs<'world> {
    pub fn get_storage_ref<C>(&self) -> Option<&C::Storage>
    where
        C: Component,
    {
        self.storages.get_ref::<C>()
    }

    pub fn move_storage_ref<C>(&mut self) -> Option<&'world C::Storage>
    where
        C: Component,
    {
        self.storages.move_ref::<C>()
    }

    pub fn move_storage_ref_mut<C>(&mut self) -> Option<&'world mut C::Storage>
    where
        C: Component,
    {
        self.storages.move_ref_mut::<C>()
    }

    #[cfg(feature = "resource")]
    pub fn get_resource_ref<R>(&self) -> Option<&R>
    where
        R: Resource,
    {
        self.resources.get_ref::<R>()
    }

    #[cfg(feature = "resource")]
    pub fn move_resource_ref<R>(&mut self) -> Option<&'world R>
    where
        R: Resource,
    {
        self.resources.move_ref::<R>()
    }

    #[cfg(feature = "resource")]
    pub fn move_resource_ref_mut<R>(&mut self) -> Option<&'world mut R>
    where
        R: Resource,
    {
        self.resources.move_ref_mut::<R>()
    }
}
