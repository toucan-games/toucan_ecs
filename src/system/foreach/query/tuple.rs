use super::*;

macro_rules! foreach_query {
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
foreach_query!(A, B, C, D, E, F, G, H, I, J, K);
foreach_query!(A, B, C, D, E, F, G, H, I, J);
foreach_query!(A, B, C, D, E, F, G, H, I);
foreach_query!(A, B, C, D, E, F, G, H);
foreach_query!(A, B, C, D, E, F, G);
foreach_query!(A, B, C, D, E, F);
foreach_query!(A, B, C, D, E);
foreach_query!(A, B, C, D);
foreach_query!(A, B, C);
foreach_query!(A, B);
foreach_query!(A);
