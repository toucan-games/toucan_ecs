use super::*;

macro_rules! foreach_fetch_mut {
    ($($types:ident),*) => {
        impl<'data, $($types),*> Fetch<'data> for ($($types,)*)
        where
            $($types: Fetch<'data>,)*
        {
            type Item = ($($types::Item,)*);

            unsafe fn new(world: WorldDataMut<'data>) -> FetchResult<Self> {
                Ok(($($types::new(world)?,)*))
            }

            #[allow(non_snake_case)]
            fn entities(&self) -> Option<Box<dyn ExactSizeIterator<Item=Entity> + Send + Sync + 'data>> {
                let ($($types,)*) = self;
                let iters = [$($types.entities(),)*];
                iters.into_iter().flatten().min_by(|x, y| x.len().cmp(&y.len()))
            }

            #[allow(non_snake_case)]
            fn fetch(&'data mut self, entity: Entity) -> FetchResult<Self::Item> {
                let ($($types,)*) = self;
                $(let $types = $types.fetch(entity)?;)*
                Ok(($($types,)*))
            }
        }
    };
}

// `FetchMut` implemented for tuples of size 12 and less
foreach_fetch_mut!(A, B, C, D, E, F, G, H, I, J, K, L);
foreach_fetch_mut!(A, B, C, D, E, F, G, H, I, J, K);
foreach_fetch_mut!(A, B, C, D, E, F, G, H, I, J);
foreach_fetch_mut!(A, B, C, D, E, F, G, H, I);
foreach_fetch_mut!(A, B, C, D, E, F, G, H);
foreach_fetch_mut!(A, B, C, D, E, F, G);
foreach_fetch_mut!(A, B, C, D, E, F);
foreach_fetch_mut!(A, B, C, D, E);
foreach_fetch_mut!(A, B, C, D);
foreach_fetch_mut!(A, B, C);
foreach_fetch_mut!(A, B);
foreach_fetch_mut!(A);
