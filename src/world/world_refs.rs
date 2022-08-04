use crate::component::{Component, RegistryRefs as StorageRefs};
#[cfg(feature = "resource")]
use crate::resource::{RegistryRefs as ResourceRefs, Resource};

pub struct WorldRefs<'data> {
    pub(super) storages: StorageRefs<'data>,
    #[cfg(feature = "resource")]
    pub(super) resources: ResourceRefs<'data>,
}

impl<'data> WorldRefs<'data> {
    pub fn get_storage_ref<C>(&self) -> Option<&C::Storage>
    where
        C: Component,
    {
        self.storages.get_ref::<C>()
    }

    pub fn move_storage_ref<C>(&mut self) -> Option<&'data C::Storage>
    where
        C: Component,
    {
        self.storages.move_ref::<C>()
    }

    pub fn move_storage_ref_mut<C>(&mut self) -> Option<&'data mut C::Storage>
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
    pub fn move_resource_ref<R>(&mut self) -> Option<&'data R>
    where
        R: Resource,
    {
        self.resources.move_ref::<R>()
    }

    #[cfg(feature = "resource")]
    pub fn move_resource_ref_mut<R>(&mut self) -> Option<&'data mut R>
    where
        R: Resource,
    {
        self.resources.move_ref_mut::<R>()
    }
}
