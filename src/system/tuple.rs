use crate::world::query::QueryMut;
use crate::world::view::ViewMut;

use super::*;

macro_rules! system {
    ($head:ident $(,)?) => {
        impl_system!($head);
    };
    ($head:ident, $($tail:ident),* $(,)?) => {
        impl_system!($head, $($tail),*);
        system!($($tail),*);
    };
}

macro_rules! impl_system {
    ($($types:ident),*) => {
        impl<'data, F, $($types),*> System<'data, ($($types,)*)> for F
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

// `System` implemented for functions with argument count of 12 and less
system!(T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12);

macro_rules! for_each_system {
    ($head:ident $(,)?) => {
        impl_for_each_system!($head);
    };
    ($head:ident, $($tail:ident),* $(,)?) => {
        impl_for_each_system!($head, $($tail),*);
        for_each_system!($($tail),*);
    };
}

macro_rules! impl_for_each_system {
    ($($types:ident),*) => {
        impl<'data, F, $($types),*> System<'data, ViewMut<'data, ($($types,)*)>> for F
        where
            F: FnMut($($types,)*) + 'data,
            ($($types,)*): QueryMut<'data>,
        {
            #[allow(non_snake_case)]
            fn run(&mut self, args: ViewMut<'data, ($($types,)*)>) {
                for item in args {
                    let ($($types,)*) = item;
                    self($($types,)*)
                }
            }
        }
    };
}

// foreach `System` implemented for functions with argument count of 12 and less
for_each_system!(T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12);
