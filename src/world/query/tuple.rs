use crate::system::foreach::QueryItem;

use super::*;

macro_rules! query {
    ($($types:ident),*) => {
        impl<'data, $($types),*> Query<'data> for ($($types,)*)
        where
            Self: From<($(QueryItem<'data, $types>,)*)>,
            $($types: Query<'data>,)*
        {}
    };
}

// `Query` implemented for tuples of size 12 and less
query!(A, B, C, D, E, F, G, H, I, J, K, L);
query!(A, B, C, D, E, F, G, H, I, J, K);
query!(A, B, C, D, E, F, G, H, I, J);
query!(A, B, C, D, E, F, G, H, I);
query!(A, B, C, D, E, F, G, H);
query!(A, B, C, D, E, F, G);
query!(A, B, C, D, E, F);
query!(A, B, C, D, E);
query!(A, B, C, D);
query!(A, B, C);
query!(A, B);
query!(A);

macro_rules! query_mut {
    ($($types:ident),*) => {
        impl<'data, $($types),*> QueryMut<'data> for ($($types,)*)
        where
            Self: From<($(QueryItem<'data, $types>,)*)>,
            $($types: QueryMut<'data>,)*
        {}
    };
}

// `QueryMut` implemented for tuples of size 12 and less
query_mut!(A, B, C, D, E, F, G, H, I, J, K, L);
query_mut!(A, B, C, D, E, F, G, H, I, J, K);
query_mut!(A, B, C, D, E, F, G, H, I, J);
query_mut!(A, B, C, D, E, F, G, H, I);
query_mut!(A, B, C, D, E, F, G, H);
query_mut!(A, B, C, D, E, F, G);
query_mut!(A, B, C, D, E, F);
query_mut!(A, B, C, D, E);
query_mut!(A, B, C, D);
query_mut!(A, B, C);
query_mut!(A, B);
query_mut!(A);

#[cfg(feature = "resource")]
macro_rules! resource_query {
    ($($types:ident),*) => {
        impl<'data, $($types),*> ResourceQuery<'data> for ($($types,)*)
        where
            Self: From<($(QueryItem<'data, $types>,)*)>,
            $($types: ResourceQuery<'data>,)*
        {}
    };
}

#[cfg(not(feature = "resource"))]
macro_rules! resource_query {
    ($($types:ident),*) => {};
}

// `ResourceQuery` implemented for tuples of size 12 and less
resource_query!(A, B, C, D, E, F, G, H, I, J, K, L);
resource_query!(A, B, C, D, E, F, G, H, I, J, K);
resource_query!(A, B, C, D, E, F, G, H, I, J);
resource_query!(A, B, C, D, E, F, G, H, I);
resource_query!(A, B, C, D, E, F, G, H);
resource_query!(A, B, C, D, E, F, G);
resource_query!(A, B, C, D, E, F);
resource_query!(A, B, C, D, E);
resource_query!(A, B, C, D);
resource_query!(A, B, C);
resource_query!(A, B);
resource_query!(A);

#[cfg(feature = "resource")]
macro_rules! resource_query_mut {
    ($($types:ident),*) => {
        impl<'data, $($types),*> ResourceQueryMut<'data> for ($($types,)*)
        where
            Self: From<($(QueryItem<'data, $types>,)*)>,
            $($types: ResourceQueryMut<'data>,)*
        {}
    };
}

#[cfg(not(feature = "resource"))]
macro_rules! resource_query_mut {
    ($($types:ident),*) => {};
}

// `QueryMut` implemented for tuples of size 12 and less
resource_query_mut!(A, B, C, D, E, F, G, H, I, J, K, L);
resource_query_mut!(A, B, C, D, E, F, G, H, I, J, K);
resource_query_mut!(A, B, C, D, E, F, G, H, I, J);
resource_query_mut!(A, B, C, D, E, F, G, H, I);
resource_query_mut!(A, B, C, D, E, F, G, H);
resource_query_mut!(A, B, C, D, E, F, G);
resource_query_mut!(A, B, C, D, E, F);
resource_query_mut!(A, B, C, D, E);
resource_query_mut!(A, B, C, D);
resource_query_mut!(A, B, C);
resource_query_mut!(A, B);
resource_query_mut!(A);
