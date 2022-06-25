use crate::component::marker::Not;
use crate::component::{Component, ViewOne, ViewOneMut};
use crate::entity::Entity;
#[cfg(feature = "resource")]
use crate::resource::{marker, Resource};
use crate::system::fetch::*;
use crate::world::{query, View, ViewMut};

use super::*;

impl QuerySealed for () {}

impl<'data> Query<'data> for () {
    type Fetch = ();
}

impl QuerySealed for Entity {}

impl<'data> Query<'data> for Entity {
    type Fetch = FetchEntity;
}

impl<'data, C> QuerySealed for &'data C where C: Component {}

impl<'data, C> Query<'data> for &'data C
where
    C: Component,
{
    type Fetch = FetchRead<C>;
}

impl<'data, C> QuerySealed for &'data mut C where C: Component {}

impl<'data, C> Query<'data> for &'data mut C
where
    C: Component,
{
    type Fetch = FetchWrite<C>;
}

impl<'data, C> QuerySealed for Option<&'data C> where C: Component {}

impl<'data, C> Query<'data> for Option<&'data C>
where
    C: Component,
{
    type Fetch = FetchOptionRead<C>;
}

impl<'data, C> QuerySealed for Option<&'data mut C> where C: Component {}

impl<'data, C> Query<'data> for Option<&'data mut C>
where
    C: Component,
{
    type Fetch = FetchOptionWrite<C>;
}

impl<C> QuerySealed for Not<C> where C: Component {}

impl<'data, C> Query<'data> for Not<C>
where
    C: Component,
{
    type Fetch = FetchNot<C>;
}

impl<'data, C> QuerySealed for ViewOne<'data, C> where C: Component {}

impl<'data, C> Query<'data> for ViewOne<'data, C>
where
    C: Component,
{
    type Fetch = FetchViewOne<C>;
}

impl<'data, C> QuerySealed for ViewOneMut<'data, C> where C: Component {}

impl<'data, C> Query<'data> for ViewOneMut<'data, C>
where
    C: Component,
{
    type Fetch = FetchViewOneMut<C>;
}

impl<'data, Q> QuerySealed for View<'data, Q> where Q: query::Query<'data> {}

impl<'data, Q> Query<'data> for View<'data, Q>
where
    Q: query::Query<'data>,
{
    type Fetch = FetchView<'data, Q>;
}

impl<'data, Q> QuerySealed for ViewMut<'data, Q> where Q: query::QueryMut<'data> {}

impl<'data, Q> Query<'data> for ViewMut<'data, Q>
where
    Q: query::QueryMut<'data>,
{
    type Fetch = FetchViewMut<'data, Q>;
}

#[cfg(feature = "resource")]
impl<'data, R> QuerySealed for marker::Resource<'data, R> where R: Resource {}

#[cfg(feature = "resource")]
impl<'data, R> Query<'data> for marker::Resource<'data, R>
where
    R: Resource,
{
    type Fetch = FetchResourceRead<R>;
}

#[cfg(feature = "resource")]
impl<'data, R> QuerySealed for marker::ResourceMut<'data, R> where R: Resource {}

#[cfg(feature = "resource")]
impl<'data, R> Query<'data> for marker::ResourceMut<'data, R>
where
    R: Resource,
{
    type Fetch = FetchResourceWrite<R>;
}

macro_rules! system_query {
    ($head:ident $(,)?) => {
        impl_system_query!($head);
    };
    ($head:ident, $($tail:ident),* $(,)?) => {
        impl_system_query!($head, $($tail),*);
        system_query!($($tail),*);
    };
}

macro_rules! impl_system_query {
    ($($types:ident),*) => {
        impl<'data, $($types),*> QuerySealed for ($($types,)*)
        where
            $($types: Query<'data>,)*
        {}

        impl<'data, $($types),*> Query<'data> for ($($types,)*)
        where
            Self: From<($(QueryItem<'data, $types>,)*)>,
            $($types: Query<'data>,)*
        {
            type Fetch = ($($types::Fetch,)*);
        }
    };
}

// `Query` implemented for functions with argument count of 12 and less
system_query!(A, B, C, D, E, F, G, H, I, J, K, L);
