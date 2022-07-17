use super::*;

macro_rules! fetch {
    ($($types:ident),*) => {
        impl<'data, $($types),*> Fetch<'data> for ($($types,)*)
        where
            $($types: Fetch<'data>,)*
        {
            type Item = ($($types::Item,)*);

            fn new(world: WorldData<'data>) -> FetchResult<Self> {
                Ok(($($types::new(world)?,)*))
            }

            #[allow(non_snake_case)]
            fn entities(&self) -> Option<Box<dyn ExactSizeIterator<Item=Entity> + Send + Sync + 'data>> {
                let ($($types,)*) = self;
                let iters = [$($types.entities(),)*];
                iters.into_iter().flatten().min_by(|x, y| x.len().cmp(&y.len()))
            }

            #[allow(non_snake_case)]
            fn fetch(&self, entity: Entity) -> FetchResult<Self::Item> {
                let ($($types,)*) = self;
                $(let $types = $types.fetch(entity)?;)*
                Ok(($($types,)*))
            }
        }
    };
}

// `Fetch` implemented for tuples of size 12 and less
fetch!(A, B, C, D, E, F, G, H, I, J, K, L);
fetch!(A, B, C, D, E, F, G, H, I, J, K);
fetch!(A, B, C, D, E, F, G, H, I, J);
fetch!(A, B, C, D, E, F, G, H, I);
fetch!(A, B, C, D, E, F, G, H);
fetch!(A, B, C, D, E, F, G);
fetch!(A, B, C, D, E, F);
fetch!(A, B, C, D, E);
fetch!(A, B, C, D);
fetch!(A, B, C);
fetch!(A, B);
fetch!(A);

macro_rules! fetch_mut {
    ($($types:ident),*) => {
        impl<'data, $($types),*> FetchMut<'data> for ($($types,)*)
        where
            $($types: FetchMut<'data>,)*
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
            fn fetch_mut(&'data mut self, entity: Entity) -> FetchResult<Self::Item> {
                let ($($types,)*) = self;
                $(let $types = $types.fetch_mut(entity)?;)*
                Ok(($($types,)*))
            }
        }
    };
}

// `FetchMut` implemented for tuples of size 12 and less
fetch_mut!(A, B, C, D, E, F, G, H, I, J, K, L);
fetch_mut!(A, B, C, D, E, F, G, H, I, J, K);
fetch_mut!(A, B, C, D, E, F, G, H, I, J);
fetch_mut!(A, B, C, D, E, F, G, H, I);
fetch_mut!(A, B, C, D, E, F, G, H);
fetch_mut!(A, B, C, D, E, F, G);
fetch_mut!(A, B, C, D, E, F);
fetch_mut!(A, B, C, D, E);
fetch_mut!(A, B, C, D);
fetch_mut!(A, B, C);
fetch_mut!(A, B);
fetch_mut!(A);
