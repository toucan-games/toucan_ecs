use super::*;

macro_rules! foreach_query {
    ($head:ident $(,)?) => {
        impl_foreach_query!($head);
    };
    ($head:ident, $($tail:ident),* $(,)?) => {
        impl_foreach_query!($head, $($tail),*);
        foreach_query!($($tail),*);
    };
}

macro_rules! impl_foreach_query {
    ($($types:ident),*) => {
        impl<'data, $($types),*> Query<'data> for ($($types,)*)
        where
            Self: From<($(QueryItem<'data, $types>,)*)>,
            $($types: Query<'data>,)*
        {
            type Fetch = ($($types::Fetch,)*);
        }
    };
}

// `ForeachQuery` implemented for functions with argument count of 12 and less
foreach_query!(A, B, C, D, E, F, G, H, I, J, K, L);
