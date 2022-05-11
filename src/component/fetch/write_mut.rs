use std::marker::PhantomData;

use crate::component::Component;
use crate::world::FetchMut;

pub struct FetchWriteMut<'data, C>
where
    C: Component,
{
    _ph: PhantomData<&'data mut C>,
}

impl<'data, C> FetchMut<'data> for FetchWriteMut<'data, C>
where
    C: Component,
{
    type Item = &'data mut C;
}
