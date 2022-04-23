use crate::component::Component;
use crate::resource::{marker::Resource as ResourceMarker, Resource};

use super::*;

impl<'data> Query<'data> for () {}

impl<'data, C> Query<'data> for &'data C where C: Component {}

impl<'data, C> Query<'data> for &'data mut C where C: Component {}

impl<'data, R> Query<'data> for ResourceMarker<&'data R> where R: Resource {}

impl<'data, R> Query<'data> for ResourceMarker<&'data mut R> where R: Resource {}

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
        {}
    };
}

// `Query` implemented for functions with argument count of 12 and less
system_query!(T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12);
