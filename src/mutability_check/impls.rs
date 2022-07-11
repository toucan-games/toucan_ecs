use crate::component::marker::Not;
use crate::component::{Component, ComponentTypeId};
use crate::entity::Entity;
#[cfg(feature = "resource")]
use crate::resource::{marker, Resource, ResourceTypeId};
use crate::world::query::{Query, QueryMut};
use crate::world::view::{View, ViewMut, ViewOne, ViewOneMut};

use super::*;

impl MutabilityCheck for () {
    const MUTABLE: bool = false;

    fn extend_before_check(_: &mut MultiMap<TypeId, bool>) {}
}

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
    const MUTABLE: bool = <&'data C>::MUTABLE;

    fn extend_before_check(multimap: &mut MultiMap<TypeId, bool>) {
        multimap.insert(ComponentTypeId::of::<C>().into(), Self::MUTABLE)
    }
}

impl<'data, C> MutabilityCheck for Option<&'data mut C>
where
    C: Component,
{
    const MUTABLE: bool = <&'data mut C>::MUTABLE;

    fn extend_before_check(multimap: &mut MultiMap<TypeId, bool>) {
        multimap.insert(ComponentTypeId::of::<C>().into(), Self::MUTABLE)
    }
}

impl<C> MutabilityCheck for Not<C>
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

#[cfg(feature = "resource")]
impl<'data, R> MutabilityCheck for Option<marker::Resource<'data, R>>
where
    R: Resource,
{
    const MUTABLE: bool = marker::Resource::<'data, R>::MUTABLE;

    fn extend_before_check(multimap: &mut MultiMap<TypeId, bool>) {
        multimap.insert(ResourceTypeId::of::<R>().into(), Self::MUTABLE)
    }
}

#[cfg(feature = "resource")]
impl<'data, R> MutabilityCheck for Option<marker::ResourceMut<'data, R>>
where
    R: Resource,
{
    const MUTABLE: bool = marker::ResourceMut::<'data, R>::MUTABLE;

    fn extend_before_check(multimap: &mut MultiMap<TypeId, bool>) {
        multimap.insert(ResourceTypeId::of::<R>().into(), Self::MUTABLE)
    }
}

impl<'data, C> MutabilityCheck for ViewOne<'data, C>
where
    C: Component,
{
    const MUTABLE: bool = false;

    fn extend_before_check(multimap: &mut MultiMap<TypeId, bool>) {
        multimap.insert(ComponentTypeId::of::<C>().into(), Self::MUTABLE);
    }
}

impl<'data, C> MutabilityCheck for ViewOneMut<'data, C>
where
    C: Component,
{
    const MUTABLE: bool = true;

    fn extend_before_check(multimap: &mut MultiMap<TypeId, bool>) {
        multimap.insert(ComponentTypeId::of::<C>().into(), Self::MUTABLE);
    }
}

impl<'data, Q> MutabilityCheck for View<'data, Q>
where
    Q: Query<'data>,
{
    const MUTABLE: bool = Q::MUTABLE;

    fn extend_before_check(multimap: &mut MultiMap<TypeId, bool>) {
        Q::extend_before_check(multimap)
    }
}

impl<'data, Q> MutabilityCheck for ViewMut<'data, Q>
where
    Q: QueryMut<'data>,
{
    const MUTABLE: bool = Q::MUTABLE;

    fn extend_before_check(multimap: &mut MultiMap<TypeId, bool>) {
        Q::extend_before_check(multimap)
    }
}
