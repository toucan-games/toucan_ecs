use super::*;

macro_rules! query {
    ($head:ident $(,)?) => {
        impl_query!($head);
    };
    ($head:ident, $($tail:ident),* $(,)?) => {
        impl_query!($head, $($tail),*);
        query!($($tail),*);
    };
}

macro_rules! impl_query {
    ($($types:ident),*) => {
        impl<'data, $($types),*> QueryShared<'data> for ($($types,)*)
        where
            $($types: QueryShared<'data>,)*
        {}

        impl<'data, $($types),*> Query<'data> for ($($types,)*)
        where
            $($types: Query<'data>,)*
        {
            type Fetch = ($($types::Fetch,)*);
        }
    };
}

// `Query` implemented for tuples of size 12 and less
query!(A, B, C, D, E, F, G, H, I, J, K, L);
