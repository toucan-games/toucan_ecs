use super::*;

macro_rules! fetch {
    ($head:ident $(,)?) => {
        impl_fetch!($head);
    };
    ($head:ident, $($tail:ident),* $(,)?) => {
        impl_fetch!($head, $($tail),*);
        fetch!($($tail),*);
    };
}

macro_rules! impl_fetch {
    ($($types:ident),*) => {
        impl<'data, $($types),*> TryFrom<&'data World> for ($($types,)*)
        where
            $($types: Fetch<'data>,)*
        {
            type Error = FetchError;

            fn try_from(world: &'data World) -> Result<Self, Self::Error> {
                Ok(($($types::try_from(world)?,)*))
            }
        }

        impl<'data, $($types),*> Fetch<'data> for ($($types,)*)
        where
            $($types: Fetch<'data>,)*
        {
            type Item = ($($types::Item,)*);

            #[allow(non_snake_case)]
            fn fetch(&self, entity: Entity) -> Result<Self::Item, FetchError> {
                let ($($types,)*) = self;
                $(let $types = $types.fetch(entity)?;)*
                Ok(($($types,)*))
            }
        }
    };
}

// `Fetch` implemented for tuples of size 12 and less
fetch!(A, B, C, D, E, F, G, H, I, J, K, L);

macro_rules! fetch_mut {
    ($head:ident $(,)?) => {
        impl_fetch_mut!($head);
    };
    ($head:ident, $($tail:ident),* $(,)?) => {
        impl_fetch_mut!($head, $($tail),*);
        fetch_mut!($($tail),*);
    };
}

macro_rules! impl_fetch_mut {
    ($($types:ident),*) => {
        impl<'data, $($types),*> FetchMut<'data> for ($($types,)*)
        where
            $($types: FetchMut<'data>,)*
        {
            type Item = ($($types::Item,)*);
        }
    };
}

// `FetchMut` implemented for tuples of size 12 and less
fetch_mut!(A, B, C, D, E, F, G, H, I, J, K, L);
