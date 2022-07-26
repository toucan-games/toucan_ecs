use crate::component::marker::Not;
use crate::component::Component;
use crate::entity::Entity;
#[cfg(feature = "resource")]
use crate::resource::{marker, Resource};
use crate::world::query::{Query, QueryMut};

impl<'data> Query<'data> for Entity {}

impl<'data> QueryMut<'data> for Entity {}

impl<'data, C> Query<'data> for &'data C where C: Component {}

impl<'data, C> QueryMut<'data> for &'data C where C: Component {}

impl<'data, C> Query<'data> for Option<&'data C> where C: Component {}

impl<'data, C> QueryMut<'data> for Option<&'data C> where C: Component {}

impl<'data, C> Query<'data> for Not<C> where C: Component {}

impl<'data, C> QueryMut<'data> for Not<C> where C: Component {}

impl<'data, C> QueryMut<'data> for &'data mut C where C: Component {}

impl<'data, C> QueryMut<'data> for Option<&'data mut C> where C: Component {}

cfg_resource! {
    impl<'data, R> Query<'data> for marker::Resource<'data, R>
    where
        R: Resource,
    {}

    impl<'data, R> QueryMut<'data> for marker::Resource<'data, R>
    where
        R: Resource,
    {}

    impl<'data, R> Query<'data> for Option<marker::Resource<'data, R>>
    where
        R: Resource,
    {}

    impl<'data, R> QueryMut<'data> for Option<marker::Resource<'data, R>>
    where
        R: Resource,
    {}

    impl<'data, R> QueryMut<'data> for marker::ResourceMut<'data, R>
    where
        R: Resource,
    {}

    impl<'data, R> QueryMut<'data> for Option<marker::ResourceMut<'data, R>>
    where
        R: Resource,
    {}
}
