use crate::{Component, Entity, Registry};

pub trait AddSet {
    fn add_set(self, registry: &mut Registry, entity: Entity);
}

mod impls {
    use super::*;

    macro_rules! add_set_impl {
        ($arg:ident | $count:tt) => ();

        ($arg:ident $(, $args:ident)+ | $idx:tt $(, $idxs:tt)+) => {
            impl<$arg$(, $args)+> AddSet for ($arg$(, $args)+)
            where
                $arg: Component$(,
                $args: Component
                )+,
            {
                fn add_set(self, registry: &mut Registry, entity: Entity) {
                    registry.add(entity, self.$idx);
                    $(registry.add(entity, self.$idxs);)*
                }
            }

            add_set_impl!($($args),+ | $($idxs),+);
        };
    }

    #[rustfmt::skip]
    add_set_impl!(L, K, J, I, H, G, F, E, D, C, B, A | 11, 10, 9, 8, 7, 6, 5, 4, 3, 2, 1, 0);
}
