use std::any::TypeId;

use crate::component::ComponentTypeId;
#[cfg(feature = "resource")]
use crate::resource::ResourceTypeId;

#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Hash)]
pub struct DataTypeId(TypeId);

impl From<ComponentTypeId> for DataTypeId {
    fn from(type_id: ComponentTypeId) -> Self {
        let type_id = type_id.into();
        Self(type_id)
    }
}

#[cfg(feature = "resource")]
impl From<ResourceTypeId> for DataTypeId {
    fn from(type_id: ResourceTypeId) -> Self {
        let type_id = type_id.into();
        Self(type_id)
    }
}

impl From<DataTypeId> for TypeId {
    fn from(type_id: DataTypeId) -> Self {
        type_id.0
    }
}
