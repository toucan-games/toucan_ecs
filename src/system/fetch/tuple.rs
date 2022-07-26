use super::*;

macro_rules! system_fetch {
    ($($types:ident),*) => {
        impl<'data, $($types),*> Fetch<'data> for ($($types,)*)
        where
            $($types: Fetch<'data>,)*
        {
            type Item = ($($types::Item,)*);

            fn fetch(world: &'data World) -> FetchResult<Self::Item> {
                Ok(($($types::fetch(world)?,)*))
            }
        }
    };
}

// System `Fetch` implemented for tuples of size 12 and less
system_fetch!(A, B, C, D, E, F, G, H, I, J, K, L);
system_fetch!(A, B, C, D, E, F, G, H, I, J, K);
system_fetch!(A, B, C, D, E, F, G, H, I, J);
system_fetch!(A, B, C, D, E, F, G, H, I);
system_fetch!(A, B, C, D, E, F, G, H);
system_fetch!(A, B, C, D, E, F, G);
system_fetch!(A, B, C, D, E, F);
system_fetch!(A, B, C, D, E);
system_fetch!(A, B, C, D);
system_fetch!(A, B, C);
system_fetch!(A, B);
system_fetch!(A);
