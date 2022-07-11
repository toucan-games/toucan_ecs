use super::*;

macro_rules! foreach_system {
    ($head:ident $(,)?) => {
        impl_foreach_system!($head);
    };
    ($head:ident, $($tail:ident),* $(,)?) => {
        impl_foreach_system!($head, $($tail),*);
        foreach_system!($($tail),*);
    };
}

macro_rules! impl_foreach_system {
    ($($types:ident),*) => {
        impl<'data, F, $($types),*> ForeachSystem<'data, ($($types,)*)> for F
        where
            F: FnMut($($types,)*) + 'data,
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
foreach_system!(T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12);
