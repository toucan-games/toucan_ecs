use crate::component::marker::Not;
use crate::component::{Component, ComponentTypeId};
use crate::entity::Entity;
#[cfg(feature = "resource")]
use crate::resource::{marker, Resource, ResourceTypeId};

use super::*;

impl MutabilityCheck for Entity {
    const MUTABLE: bool = false;

    fn extend_before_check(_: &mut MultiMap<TypeId, bool>) {}
}

impl<'data, C> MutabilityCheck for &'data C
where
    C: Component,
{
    const MUTABLE: bool = false;

    fn extend_before_check(multimap: &mut MultiMap<TypeId, bool>) {
        multimap.insert(ComponentTypeId::of::<C>().into(), Self::MUTABLE)
    }
}

impl<'data, C> MutabilityCheck for &'data mut C
where
    C: Component,
{
    const MUTABLE: bool = true;

    fn extend_before_check(multimap: &mut MultiMap<TypeId, bool>) {
        multimap.insert(ComponentTypeId::of::<C>().into(), Self::MUTABLE)
    }
}

impl<'data, C> MutabilityCheck for Option<&'data C>
where
    C: Component,
{
    const MUTABLE: bool = false;

    fn extend_before_check(multimap: &mut MultiMap<TypeId, bool>) {
        multimap.insert(ComponentTypeId::of::<C>().into(), Self::MUTABLE)
    }
}

impl<'data, C> MutabilityCheck for Option<&'data mut C>
where
    C: Component,
{
    const MUTABLE: bool = true;

    fn extend_before_check(multimap: &mut MultiMap<TypeId, bool>) {
        multimap.insert(ComponentTypeId::of::<C>().into(), Self::MUTABLE)
    }
}

impl<'data, C> MutabilityCheck for Not<C>
where
    C: Component,
{
    const MUTABLE: bool = false;

    fn extend_before_check(multimap: &mut MultiMap<TypeId, bool>) {
        multimap.insert(ComponentTypeId::of::<C>().into(), Self::MUTABLE)
    }
}

#[cfg(feature = "resource")]
impl<'data, R> MutabilityCheck for marker::Resource<'data, R>
where
    R: Resource,
{
    const MUTABLE: bool = false;

    fn extend_before_check(multimap: &mut MultiMap<TypeId, bool>) {
        multimap.insert(ResourceTypeId::of::<R>().into(), Self::MUTABLE)
    }
}

#[cfg(feature = "resource")]
impl<'data, R> MutabilityCheck for marker::ResourceMut<'data, R>
where
    R: Resource,
{
    const MUTABLE: bool = true;

    fn extend_before_check(multimap: &mut MultiMap<TypeId, bool>) {
        multimap.insert(ResourceTypeId::of::<R>().into(), Self::MUTABLE)
    }
}
