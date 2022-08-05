use std::collections::HashMap;
use std::hash::BuildHasherDefault;

use crate::component::storage::ErasedStorageHolder;
use crate::component::{Component, ComponentTypeId, Registry};
use crate::hash::TypeIdHasher;
use crate::ref_kind::RefKind;

type StorageRefKind<'data> = RefKind<'data, ErasedStorageHolder>;

#[repr(transparent)]
pub struct RegistryRefs<'data> {
    refs: HashMap<ComponentTypeId, Option<StorageRefKind<'data>>, BuildHasherDefault<TypeIdHasher>>,
}

impl<'data> From<&'data Registry> for RegistryRefs<'data> {
    fn from(registry: &'data Registry) -> Self {
        let refs = registry
            .iter()
            .map(|(&type_id, erased)| (type_id, Some(StorageRefKind::Ref(erased))))
            .collect();
        Self { refs }
    }
}

impl<'data> From<&'data mut Registry> for RegistryRefs<'data> {
    fn from(registry: &'data mut Registry) -> Self {
        let refs = registry
            .iter_mut()
            .map(|(&type_id, erased)| (type_id, Some(StorageRefKind::Mut(erased))))
            .collect();
        Self { refs }
    }
}

impl<'data> RegistryRefs<'data> {
    #[cfg(feature = "resource")]
    pub fn empty() -> Self {
        let refs = HashMap::default();
        Self { refs }
    }

    pub fn get_ref<C>(&self) -> Option<&C::Storage>
    where
        C: Component,
    {
        let type_id = ComponentTypeId::of::<C>();
        let storage = self.refs.get(&type_id)?.as_ref();
        let storage = storage.expect("storage was already borrowed mutably");
        let storage = match storage {
            StorageRefKind::Ref(storage) => *storage,
            StorageRefKind::Mut(storage) => &**storage,
        };
        let storage = storage.as_storage_ref().expect("downcast error");
        Some(storage)
    }

    pub fn move_ref<C>(&mut self) -> Option<&'data C::Storage>
    where
        C: Component,
    {
        let type_id = ComponentTypeId::of::<C>();
        let storage = self.refs.get(&type_id)?.as_ref();
        let storage = storage.expect("storage was already borrowed mutably");
        let storage = match storage {
            StorageRefKind::Ref(storage) => *storage,
            StorageRefKind::Mut(_) => {
                let storage = self.refs.remove(&type_id)?;
                let storage = storage.expect("storage was already borrowed mutably");
                match storage {
                    StorageRefKind::Ref(_) => unreachable!(),
                    StorageRefKind::Mut(storage) => {
                        let storage = &*storage;
                        let ref_kind = Some(StorageRefKind::Ref(storage));
                        self.refs.insert(type_id, ref_kind);
                        storage
                    }
                }
            }
        };
        let storage = storage.as_storage_ref().expect("downcast error");
        Some(storage)
    }

    pub fn move_ref_mut<C>(&mut self) -> Option<&'data mut C::Storage>
    where
        C: Component,
    {
        let type_id = ComponentTypeId::of::<C>();
        let storage = self.refs.remove(&type_id)?;
        let storage = storage.expect("storage was already borrowed mutably");
        let storage = match storage {
            StorageRefKind::Ref(storage) => {
                let ref_kind = Some(StorageRefKind::Ref(storage));
                self.refs.insert(type_id, ref_kind);
                move_mut_failed()
            }
            StorageRefKind::Mut(storage) => {
                self.refs.insert(type_id, None);
                storage
            }
        };
        let storage = storage.as_storage_mut().expect("downcast error");
        Some(storage)
    }
}

#[cold]
#[track_caller]
fn move_mut_failed() -> ! {
    panic!("storage was already borrowed immutably")
}
