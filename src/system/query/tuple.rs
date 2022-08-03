use super::*;

macro_rules! system_query {
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

// `Query` implemented for functions with argument count of 12 and less
system_query!(A, B, C, D, E, F, G, H, I, J, K, L);
system_query!(A, B, C, D, E, F, G, H, I, J, K);
system_query!(A, B, C, D, E, F, G, H, I, J);
system_query!(A, B, C, D, E, F, G, H, I);
system_query!(A, B, C, D, E, F, G, H);
system_query!(A, B, C, D, E, F, G);
system_query!(A, B, C, D, E, F);
system_query!(A, B, C, D, E);
system_query!(A, B, C, D);
system_query!(A, B, C);
system_query!(A, B);
system_query!(A);
