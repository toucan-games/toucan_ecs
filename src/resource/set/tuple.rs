use super::*;

macro_rules! resource_set {
    ($($types:ident),*) => {
        impl<$($types),*> ResourceSet for ($($types,)*)
        where
            $($types: Resource,)*
        {
            #[allow(non_snake_case)]
            fn create(self, registry: &mut Registry) {
                let ($($types,)*) = self;
                $($types.create(registry);)*
            }

            fn destroy(registry: &mut Registry) {
                $($types::destroy(registry);)*
            }

            fn contains(registry: &Registry) -> bool {
                $($types::contains(registry))&&*
            }
        }
    }
}

// `ResourceSet` implemented for tuples of size 12 and less
resource_set!(A, B, C, D, E, F, G, H, I, J, K, L);
resource_set!(A, B, C, D, E, F, G, H, I, J, K);
resource_set!(A, B, C, D, E, F, G, H, I, J);
resource_set!(A, B, C, D, E, F, G, H, I);
resource_set!(A, B, C, D, E, F, G, H);
resource_set!(A, B, C, D, E, F, G);
resource_set!(A, B, C, D, E, F);
resource_set!(A, B, C, D, E);
resource_set!(A, B, C, D);
resource_set!(A, B, C);
resource_set!(A, B);
resource_set!(A);
