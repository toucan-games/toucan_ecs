use std::any::TypeId;

use super::Component;

#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Hash)]
pub struct ComponentTypeId(TypeId);

impl ComponentTypeId {
    pub fn of<C>() -> Self
    where
        C: Component,
    {
        Self(TypeId::of::<C>())
    }
}
