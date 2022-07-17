use super::*;

macro_rules! system {
    ($($types:ident),*) => {
        impl<'data, Fn, $($types),*> System<'data, ($($types,)*)> for Fn
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

// `System` implemented for functions with argument count of 12 and less
system!(A, B, C, D, E, F, G, H, I, J, K, L);
system!(A, B, C, D, E, F, G, H, I, J, K);
system!(A, B, C, D, E, F, G, H, I, J);
system!(A, B, C, D, E, F, G, H, I);
system!(A, B, C, D, E, F, G, H);
system!(A, B, C, D, E, F, G);
system!(A, B, C, D, E, F);
system!(A, B, C, D, E);
system!(A, B, C, D);
system!(A, B, C);
system!(A, B);
system!(A);
