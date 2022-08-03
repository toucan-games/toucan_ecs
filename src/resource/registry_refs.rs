use std::collections::HashMap;
use std::hash::BuildHasherDefault;

use crate::hash::TypeIdHasher;
use crate::ref_kind::RefKind;
use crate::resource::erased::ErasedResourceHolder;
use crate::resource::{Registry, Resource, ResourceTypeId};

type ResourceRefKind<'data> = RefKind<'data, ErasedResourceHolder>;

#[repr(transparent)]
pub struct RegistryRefs<'data> {
    refs: HashMap<ResourceTypeId, Option<ResourceRefKind<'data>>, BuildHasherDefault<TypeIdHasher>>,
}

impl<'data> From<&'data Registry> for RegistryRefs<'data> {
    fn from(registry: &'data Registry) -> Self {
        let refs = registry
            .iter()
            .map(|(&type_id, erased)| (type_id, Some(ResourceRefKind::Ref(erased))))
            .collect();
        Self { refs }
    }
}

impl<'data> From<&'data mut Registry> for RegistryRefs<'data> {
    fn from(registry: &'data mut Registry) -> Self {
        let refs = registry
            .iter_mut()
            .map(|(&type_id, erased)| (type_id, Some(ResourceRefKind::Mut(erased))))
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
        let resource = self.refs.get(&type_id)?.as_ref();
        let resource = resource.expect("storage was already borrowed mutably");
        let resource = match resource {
            ResourceRefKind::Ref(resource) => *resource,
            ResourceRefKind::Mut(resource) => &**resource,
        };
        let resource = resource.as_resource_ref().expect("downcast error");
        Some(resource)
    }

    pub fn move_ref<R>(&mut self) -> Option<&'data R>
    where
        R: Resource,
    {
        let type_id = ResourceTypeId::of::<R>();
        let resource = self.refs.get(&type_id)?.as_ref();
        let resource = resource.expect("resource was already borrowed mutably");
        let resource = match resource {
            ResourceRefKind::Ref(resource) => *resource,
            ResourceRefKind::Mut(_) => {
                let resource = self.refs.remove(&type_id)?;
                let resource = resource.expect("resource was already borrowed mutably");
                match resource {
                    ResourceRefKind::Ref(_) => unreachable!(),
                    ResourceRefKind::Mut(resource) => {
                        let resource = &*resource;
                        let ref_kind = Some(ResourceRefKind::Ref(resource));
                        self.refs.insert(type_id, ref_kind);
                        resource
                    }
                }
            }
        };
        let resource = resource.as_resource_ref().expect("downcast error");
        Some(resource)
    }

    pub fn move_ref_mut<R>(&mut self) -> Option<&'data mut R>
    where
        R: Resource,
    {
        let type_id = ResourceTypeId::of::<R>();
        let resource = self.refs.remove(&type_id)?;
        let resource = resource.expect("resource was already borrowed mutably");
        let resource = match resource {
            ResourceRefKind::Ref(resource) => {
                let ref_kind = Some(ResourceRefKind::Ref(resource));
                self.refs.insert(type_id, ref_kind);
                move_mut_failed()
            }
            ResourceRefKind::Mut(resource) => {
                self.refs.insert(type_id, None);
                resource
            }
        };
        let resource = resource.as_resource_mut().expect("downcast error");
        Some(resource)
    }
}

#[cold]
#[track_caller]
fn move_mut_failed() -> ! {
    panic!("resource was already borrowed immutably")
}
