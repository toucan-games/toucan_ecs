use super::*;

impl<'data> Fetch<'data> for () {
    type Item = ();
}

macro_rules! system_fetch {
    ($head:ident $(,)?) => {
        impl_system_fetch!($head);
    };
    ($head:ident, $($tail:ident),* $(,)?) => {
        impl_system_fetch!($head, $($tail),*);
        system_fetch!($($tail),*);
    };
}

macro_rules! impl_system_fetch {
    ($($types:ident),*) => {
        impl<'data, $($types),*> Fetch<'data> for ($($types,)*)
        where
            $($types: Fetch<'data>,)*
        {
            type Item = ($($types::Item,)*);
        }
    };
}

// System `Fetch` implemented for tuples of size 12 and less
system_fetch!(A, B, C, D, E, F, G, H, I, J, K, L);