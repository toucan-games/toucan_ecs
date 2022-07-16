use std::any::TypeId;

use super::Resource;

#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Hash)]
pub struct ResourceTypeId(TypeId);

impl ResourceTypeId {
    pub fn of<R>() -> Self
    where
        R: Resource,
    {
        Self(TypeId::of::<R>())
    }
}
