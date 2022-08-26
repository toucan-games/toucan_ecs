use crate::component::{Component, ComponentTypeId};
use crate::entity::Entity;
use crate::marker::*;
#[cfg(feature = "resource")]
use crate::resource::{Resource, ResourceTypeId};
use crate::system::foreach::{ForeachHolder, Query as ForeachQuery};
use crate::world::query::{Query, QueryMut};
use crate::world::view::{View, ViewMut, ViewOne, ViewOneMut};

use super::*;

impl MutabilityCheck for () {
    const LENGTH: usize = 0;

    fn check(_: &mut CheckMap) {}
}

impl MutabilityCheck for Entity {
    const LENGTH: usize = 0;

    fn check(_: &mut CheckMap) {}
}

impl<'data, C> MutabilityCheck for &'data C
where
    C: Component,
{
    const LENGTH: usize = 1;

    fn check(check_map: &mut CheckMap) {
        let mutability = Mutability::Immutable;
        let type_id = ComponentTypeId::of::<C>().into();
        let prev = check_map.insert(type_id, mutability);
        match prev {
            Some(Mutability::Immutable) => (),
            Some(Mutability::Mutable) => {
                let type_name = core::any::type_name::<C>();
                panic!("immutable and mutable borrows occur for {}", type_name)
            }
            None => (),
        }
    }
}

impl<'data, C> MutabilityCheck for &'data mut C
where
    C: Component,
{
    const LENGTH: usize = 1;

    fn check(check_map: &mut CheckMap) {
        let mutability = Mutability::Mutable;
        let type_id = ComponentTypeId::of::<C>().into();
        let prev = check_map.insert(type_id, mutability);
        match prev {
            Some(_) => {
                let type_name = core::any::type_name::<C>();
                panic!("multiple mutable borrows occur for {}", type_name)
            }
            None => (),
        }
    }
}

impl<'data, C> MutabilityCheck for Option<&'data C>
where
    C: Component,
{
    const LENGTH: usize = 1;

    fn check(check_map: &mut CheckMap) {
        <&C as MutabilityCheck>::check(check_map)
    }
}

impl<'data, C> MutabilityCheck for Option<&'data mut C>
where
    C: Component,
{
    const LENGTH: usize = 1;

    fn check(check_map: &mut CheckMap) {
        <&mut C as MutabilityCheck>::check(check_map)
    }
}

impl<C> MutabilityCheck for Not<C>
where
    C: Component,
{
    const LENGTH: usize = 1;

    fn check(check_map: &mut CheckMap) {
        <&C as MutabilityCheck>::check(check_map)
    }
}

#[cfg(feature = "resource")]
impl<'data, R> MutabilityCheck for Res<'data, R>
where
    R: Resource,
{
    const LENGTH: usize = 1;

    fn check(check_map: &mut CheckMap) {
        let mutability = Mutability::Immutable;
        let type_id = ResourceTypeId::of::<R>().into();
        let prev = check_map.insert(type_id, mutability);
        match prev {
            Some(Mutability::Immutable) => (),
            Some(Mutability::Mutable) => {
                let type_name = core::any::type_name::<R>();
                panic!("immutable and mutable borrows occur for {}", type_name)
            }
            None => (),
        }
    }
}

#[cfg(feature = "resource")]
impl<'data, R> MutabilityCheck for ResMut<'data, R>
where
    R: Resource,
{
    const LENGTH: usize = 1;

    fn check(check_map: &mut CheckMap) {
        let mutability = Mutability::Mutable;
        let type_id = ResourceTypeId::of::<R>().into();
        let prev = check_map.insert(type_id, mutability);
        match prev {
            Some(_) => {
                let type_name = core::any::type_name::<R>();
                panic!("multiple mutable borrows occur for {}", type_name)
            }
            None => (),
        }
    }
}

#[cfg(feature = "resource")]
impl<'data, R> MutabilityCheck for Option<Res<'data, R>>
where
    R: Resource,
{
    const LENGTH: usize = 1;

    fn check(check_map: &mut CheckMap) {
        <Res<R> as MutabilityCheck>::check(check_map)
    }
}

#[cfg(feature = "resource")]
impl<'data, R> MutabilityCheck for Option<ResMut<'data, R>>
where
    R: Resource,
{
    const LENGTH: usize = 1;

    fn check(check_map: &mut CheckMap) {
        <ResMut<R> as MutabilityCheck>::check(check_map)
    }
}

impl<'data, C> MutabilityCheck for ViewOne<'data, C>
where
    C: Component,
{
    const LENGTH: usize = 1;

    fn check(check_map: &mut CheckMap) {
        <&C as MutabilityCheck>::check(check_map)
    }
}

impl<'data, C> MutabilityCheck for ViewOneMut<'data, C>
where
    C: Component,
{
    const LENGTH: usize = 1;

    fn check(check_map: &mut CheckMap) {
        <&mut C as MutabilityCheck>::check(check_map)
    }
}

impl<'data, Q> MutabilityCheck for View<'data, Q>
where
    Q: Query<'data>,
{
    const LENGTH: usize = Q::LENGTH;

    fn check(check_map: &mut CheckMap) {
        Q::check(check_map)
    }
}

impl<'data, Q> MutabilityCheck for ViewMut<'data, Q>
where
    Q: QueryMut<'data>,
{
    const LENGTH: usize = Q::LENGTH;

    fn check(check_map: &mut CheckMap) {
        Q::check(check_map)
    }
}

impl<'data, Q> MutabilityCheck for ForeachHolder<'data, Q>
where
    Q: ForeachQuery<'data>,
{
    const LENGTH: usize = Q::LENGTH;

    fn check(check_map: &mut CheckMap) {
        Q::check(check_map)
    }
}
