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
        impl<F, $($types),*> System<($($types,)*)> for F
        where
            F: FnMut($($types,)*),
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

impl<F> System<()> for F
where
    F: FnMut(),
{
    fn run(&mut self, _: ()) {
        self()
    }
}
