use super::*;

macro_rules! viewable {
    ($head:ident $(,)?) => {
        impl_viewable!($head);
    };
    ($head:ident, $($tail:ident),* $(,)?) => {
        impl_viewable!($head, $($tail),*);
        viewable!($($tail),*);
    };
}

macro_rules! impl_viewable {
    ($($types:ident),*) => {
        impl<'data, $($types),*> SharedViewable<'data> for ($($types,)*)
        where
            $($types: SharedViewable<'data>,)*
        {}

        impl<'data, $($types),*> Viewable<'data> for ($($types,)*)
        where
            $($types: Viewable<'data>,)*
        {
            type Fetch = ($($types::Fetch,)*);
        }
    };
}

// `Viewable` implemented for tuples of size 12 and less
viewable!(A, B, C, D, E, F, G, H, I, J, K, L);
