use std::collections::HashMap;
use std::hash::BuildHasherDefault;

use ref_kind::{Many, RefKind};

use crate::hash::TypeIdHasher;
use crate::resource::erased::ErasedResourceHolder;
use crate::resource::{Registry, Resource, ResourceTypeId};

#[repr(transparent)]
#[derive(Default)]
pub struct RegistryRefs<'data> {
    refs: HashMap<
        ResourceTypeId,
        Option<RefKind<'data, ErasedResourceHolder>>,
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
    pub fn get_ref<R>(&self) -> Option<&R>
    where
        R: Resource,
    {
        let type_id = ResourceTypeId::of::<R>();
        let erased = self.refs.get(&type_id)?.as_ref().unwrap().get_ref();
        let resource = erased.as_resource_ref().expect("downcast error");
        Some(resource)
    }

    pub fn move_ref<R>(&mut self) -> Option<&'data R>
    where
        R: Resource,
    {
        let type_id = ResourceTypeId::of::<R>();
        let erased = self.refs.move_ref(type_id)?;
        let resource = erased.as_resource_ref().expect("downcast error");
        Some(resource)
    }

    pub fn move_mut<R>(&mut self) -> Option<&'data mut R>
    where
        R: Resource,
    {
        let type_id = ResourceTypeId::of::<R>();
        let erased = self.refs.move_mut(type_id)?;
        let resource = erased.as_resource_mut().expect("downcast error");
        Some(resource)
    }
}
