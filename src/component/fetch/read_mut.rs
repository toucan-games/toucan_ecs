use std::marker::PhantomData;

use crate::component::Component;
use crate::world::FetchMut;

pub struct FetchReadMut<'data, C>
where
    C: Component,
{
    _ph: PhantomData<&'data C>,
}

impl<'data, C> FetchMut<'data> for FetchReadMut<'data, C>
where
    C: Component,
{
    type Item = &'data C;
}
