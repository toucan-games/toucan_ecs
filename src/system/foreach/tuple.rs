use super::*;

macro_rules! foreach_system {
    ($($types:ident),*) => {
        impl<'data, Fn, $($types),*> ForeachSystem<'data, ($($types,)*)> for Fn
        where
            Fn: FnMut($($types,)*) + 'data,
            ($($types,)*): Query<'data>,
        {
            #[allow(non_snake_case)]
            fn run(&mut self, args: ($($types,)*)) {
                let ($($types,)*) = args;
                self($($types,)*)
            }
        }
    };
}

// `ForEachSystem` is implemented for functions with argument count of 12 and less
foreach_system!(A, B, C, D, E, F, G, H, I, J, K, L);
foreach_system!(A, B, C, D, E, F, G, H, I, J, K);
foreach_system!(A, B, C, D, E, F, G, H, I, J);
foreach_system!(A, B, C, D, E, F, G, H, I);
foreach_system!(A, B, C, D, E, F, G, H);
foreach_system!(A, B, C, D, E, F, G);
foreach_system!(A, B, C, D, E, F);
foreach_system!(A, B, C, D, E);
foreach_system!(A, B, C, D);
foreach_system!(A, B, C);
foreach_system!(A, B);
foreach_system!(A);
