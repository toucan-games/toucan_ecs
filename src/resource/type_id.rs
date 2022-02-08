use std::any::TypeId;

use super::Resource;

#[repr(transparent)]
#[derive(Eq, PartialEq, Hash)]
pub struct ResourceTypeId(TypeId);

impl ResourceTypeId {
    pub fn of<R>() -> Self
    where
        R: Resource,
    {
        Self(TypeId::of::<R>())
    }
}
