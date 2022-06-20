use crate::component::Component;
#[cfg(feature = "resource")]
use crate::resource::{marker::Resource as ResourceMarker, Resource};
use crate::system::fetch::*;

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

#[cfg(feature = "resource")]
impl<'data, R> Query<'data> for ResourceMarker<&'data R>
where
    R: Resource,
{
    type Fetch = FetchResourceRead<R>;
}

#[cfg(feature = "resource")]
impl<'data, R> Query<'data> for ResourceMarker<&'data mut R>
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
