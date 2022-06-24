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
        impl<'data, $($types),*> QuerySealed for ($($types,)*)
        where
            $($types: Query<'data>,)*
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

macro_rules! query_mut {
    ($head:ident $(,)?) => {
        impl_query_mut!($head);
    };
    ($head:ident, $($tail:ident),* $(,)?) => {
        impl_query_mut!($head, $($tail),*);
        query_mut!($($tail),*);
    };
}

macro_rules! impl_query_mut {
    ($($types:ident),*) => {
        impl<'data, $($types),*> SoundnessCheck for ($($types,)*)
        where
            $($types: QueryMut<'data>,)*
        {
            const MUTABLE: bool = $($types::MUTABLE)||*;

            fn extend_before_check(multimap: &mut MultiMap<TypeId, bool>) {
                $($types::extend_before_check(multimap);)*
            }
        }

        impl<'data, $($types),*> QueryMut<'data> for ($($types,)*)
        where
            $($types: QueryMut<'data>,)*
        {
            type Fetch = ($($types::Fetch,)*);
        }
    };
}

// `QueryMut` implemented for tuples of size 12 and less
query_mut!(A, B, C, D, E, F, G, H, I, J, K, L);
