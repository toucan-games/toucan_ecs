use crate::component::ComponentTypeId;
#[cfg(feature = "resource")]
use crate::resource::ResourceTypeId;

#[derive(Eq, PartialEq, Hash)]
pub enum TypeId {
    Component(ComponentTypeId),
    #[cfg(feature = "resource")]
    Resource(ResourceTypeId),
}

impl From<ComponentTypeId> for TypeId {
    fn from(type_id: ComponentTypeId) -> Self {
        Self::Component(type_id)
    }
}

cfg_resource! {
    impl From<ResourceTypeId> for TypeId {
        fn from(type_id: ResourceTypeId) -> Self {
            Self::Resource(type_id)
        }
    }
}
