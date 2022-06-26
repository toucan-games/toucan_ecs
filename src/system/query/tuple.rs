use super::*;

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
