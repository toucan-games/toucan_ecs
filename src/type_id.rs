use crate::component::ComponentTypeId;
#[cfg(feature = "resource")]
use crate::resource::ResourceTypeId;

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Hash)]
pub enum DataTypeId {
    Component(ComponentTypeId),
    #[cfg(feature = "resource")]
    Resource(ResourceTypeId),
}

impl From<ComponentTypeId> for DataTypeId {
    fn from(type_id: ComponentTypeId) -> Self {
        Self::Component(type_id)
    }
}

cfg_resource! {
    impl From<ResourceTypeId> for DataTypeId {
        fn from(type_id: ResourceTypeId) -> Self {
            Self::Resource(type_id)
        }
    }
}
