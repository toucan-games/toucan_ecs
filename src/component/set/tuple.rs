use super::*;

macro_rules! component_set {
    ($($types:ident),*) => {
        impl<$($types),*> ComponentSet for ($($types,)*)
        where
            $($types: Component,)*
        {
            #[allow(non_snake_case)]
            fn attach(self, registry: &mut Registry, entity: Entity) {
                let ($($types,)*) = self;
                $($types.attach(registry, entity);)*
            }

            fn remove(registry: &mut Registry, entity: Entity) {
                $($types::remove(registry, entity);)*
            }

            fn attached(registry: &Registry, entity: Entity) -> bool {
                $($types::attached(registry, entity))&&*
            }
        }
    }
}

// `ComponentSet` implemented for tuples of size 12 and less
component_set!(A, B, C, D, E, F, G, H, I, J, K, L);
component_set!(A, B, C, D, E, F, G, H, I, J, K);
component_set!(A, B, C, D, E, F, G, H, I, J);
component_set!(A, B, C, D, E, F, G, H, I);
component_set!(A, B, C, D, E, F, G, H);
component_set!(A, B, C, D, E, F, G);
component_set!(A, B, C, D, E, F);
component_set!(A, B, C, D, E);
component_set!(A, B, C, D);
component_set!(A, B, C);
component_set!(A, B);
component_set!(A);
