use crate::component::{Component, ViewOne, ViewOneMut};
#[cfg(feature = "resource")]
use crate::resource::{
    marker::{Resource as ResourceMarker, ResourceMut as ResourceMarkerMut},
    Resource,
};
use crate::system::fetch::*;
use crate::world::{Query as WorldQuery, QueryMut as WorldQueryMut, View, ViewMut};

use super::*;

impl<'data> Query<'data> for () {
    type Fetch = ();
}

impl<'data, C> Query<'data> for &'data C
where
    C: Component,
{
    type Fetch = FetchRead<C>;
}

impl<'data, C> Query<'data> for &'data mut C
where
    C: Component,
{
    type Fetch = FetchWrite<C>;
}

impl<'data, C> Query<'data> for Option<&'data C>
where
    C: Component,
{
    type Fetch = FetchOptionRead<C>;
}

impl<'data, C> Query<'data> for Option<&'data mut C>
where
    C: Component,
{
    type Fetch = FetchOptionWrite<C>;
}

impl<'data, C> Query<'data> for ViewOne<'data, C>
where
    C: Component,
{
    type Fetch = FetchViewOne<C>;
}

impl<'data, C> Query<'data> for ViewOneMut<'data, C>
where
    C: Component,
{
    type Fetch = FetchViewOneMut<C>;
}

impl<'data, Q> Query<'data> for View<'data, Q>
where
    Q: WorldQuery<'data>,
{
    type Fetch = FetchView<'data, Q>;
}

impl<'data, Q> Query<'data> for ViewMut<'data, Q>
where
    Q: WorldQueryMut<'data>,
{
    type Fetch = FetchViewMut<'data, Q>;
}

#[cfg(feature = "resource")]
impl<'data, R> Query<'data> for ResourceMarker<'data, R>
where
    R: Resource,
{
    type Fetch = FetchResourceRead<R>;
}

#[cfg(feature = "resource")]
impl<'data, R> Query<'data> for ResourceMarkerMut<'data, R>
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
        impl<'data, $($types),*> Query<'data> for ($($types,)*)
        where
            $($types: Query<'data>,)*
        {
            type Fetch = ($($types::Fetch,)*);
        }
    };
}

// `Query` implemented for functions with argument count of 12 and less
system_query!(A, B, C, D, E, F, G, H, I, J, K, L);
