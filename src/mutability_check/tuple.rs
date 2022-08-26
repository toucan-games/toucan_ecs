use super::*;

use tupleops::TupleLength;

macro_rules! mutability_check {
    ($($types:ident),*) => {
        impl<$($types),*> MutabilityCheck for ($($types,)*)
        where
            $($types: MutabilityCheck,)*
        {
            const LENGTH: usize = <Self as TupleLength>::LENGTH;

            fn check(check_map: &mut CheckMap) {
                $($types::check(check_map);)*
            }
        }
    };
}

// `MutabilityCheck` is implemented for tuples of size 12 and less
mutability_check!(A, B, C, D, E, F, G, H, I, J, K, L);
mutability_check!(A, B, C, D, E, F, G, H, I, J, K);
mutability_check!(A, B, C, D, E, F, G, H, I, J);
mutability_check!(A, B, C, D, E, F, G, H, I);
mutability_check!(A, B, C, D, E, F, G, H);
mutability_check!(A, B, C, D, E, F, G);
mutability_check!(A, B, C, D, E, F);
mutability_check!(A, B, C, D, E);
mutability_check!(A, B, C, D);
mutability_check!(A, B, C);
mutability_check!(A, B);
mutability_check!(A);
