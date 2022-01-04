use crate::{Entity, Registry};

pub trait ComponentSet {
    fn attach(self, registry: &mut Registry, entity: Entity);
}

mod tuple {
    use crate::Component;

    use super::*;

    macro_rules! component_set {
        ($head:ident) => {
            impl_component_set!($head);
        };
        ($head:ident, $($tail:ident),*) => {
            impl_component_set!($head, $($tail),*);
            component_set!($($tail),*);
        };
    }

    macro_rules! impl_component_set {
        ($($types:ident),*) => {
            impl<$($types),*> ComponentSet for ($($types,)*)
            where
                $($types: Component,)*
            {
                #[allow(non_snake_case)]
                fn attach(self, registry: &mut Registry, entity: Entity) {
                    let ($($types,)*) = self;
                    $(registry.attach(entity, $types);)*
                }
            }
        }
    }

    // `ComponentSet` implemented for tuples of size 12 and less
    component_set!(A, B, C, D, E, F, G, H, I, J, K, L);
}
