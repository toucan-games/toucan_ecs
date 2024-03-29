use std::hash::BuildHasherDefault;

use hashbrown::HashMap;
use ref_kind::{Many, RefKind};

use crate::component::storage::ErasedStorageHolder;
use crate::component::{Component, ComponentTypeId, Registry};
use crate::hash::TypeIdHasher;

#[repr(transparent)]
#[derive(Default)]
pub struct RegistryRefs<'data> {
    refs: HashMap<
        ComponentTypeId,
        Option<RefKind<'data, ErasedStorageHolder>>,
        BuildHasherDefault<TypeIdHasher>,
    >,
}

impl<'data> From<&'data Registry> for RegistryRefs<'data> {
    fn from(registry: &'data Registry) -> Self {
        let refs = registry
            .iter()
            .map(|(&type_id, erased)| (type_id, Some(RefKind::Ref(erased))))
            .collect();
        Self { refs }
    }
}

impl<'data> From<&'data mut Registry> for RegistryRefs<'data> {
    fn from(registry: &'data mut Registry) -> Self {
        let refs = registry
            .iter_mut()
            .map(|(&type_id, erased)| (type_id, Some(RefKind::Mut(erased))))
            .collect();
        Self { refs }
    }
}

impl<'data> RegistryRefs<'data> {
    pub fn get_ref<C>(&self) -> Option<&C::Storage>
    where
        C: Component,
    {
        let type_id = ComponentTypeId::of::<C>();
        let erased = self.refs.get(&type_id)?.as_ref().unwrap().get_ref();
        let storage = erased.as_storage_ref().expect("downcast error");
        Some(storage)
    }

    pub fn move_ref<C>(&mut self) -> Option<&'data C::Storage>
    where
        C: Component,
    {
        let type_id = ComponentTypeId::of::<C>();
        let erased = self.refs.move_ref(type_id)?;
        let storage = erased.as_storage_ref().expect("downcast error");
        Some(storage)
    }

    pub fn move_mut<C>(&mut self) -> Option<&'data mut C::Storage>
    where
        C: Component,
    {
        let type_id = ComponentTypeId::of::<C>();
        let erased = self.refs.move_mut(type_id)?;
        let storage = erased.as_storage_mut().expect("downcast error");
        Some(storage)
    }
}
