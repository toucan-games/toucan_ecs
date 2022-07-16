use super::*;

macro_rules! mutability_check {
    ($head:ident $(,)?) => {
        impl_mutability_check!($head);
    };
    ($head:ident, $($tail:ident),* $(,)?) => {
        impl_mutability_check!($head, $($tail),*);
        mutability_check!($($tail),*);
    };
}

macro_rules! impl_mutability_check {
    ($($types:ident),*) => {
        impl<$($types),*> MutabilityCheck for ($($types,)*)
        where
            $($types: MutabilityCheck,)*
        {
            const MUTABLE: bool = $($types::MUTABLE)||*;

            fn extend_before_check(multimap: &mut MultiMap<DataTypeId, bool>) {
                $($types::extend_before_check(multimap);)*
            }
        }
    };
}

// `MutabilityCheck` is implemented for tuples of size 12 and less
mutability_check!(A, B, C, D, E, F, G, H, I, J, K, L);
