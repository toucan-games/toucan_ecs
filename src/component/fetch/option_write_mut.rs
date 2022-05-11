use std::marker::PhantomData;

use crate::component::Component;
use crate::world::FetchMut;

pub struct FetchOptionWriteMut<'data, C>
where
    C: Component,
{
    _ph: PhantomData<Option<&'data mut C>>,
}

impl<'data, C> FetchMut<'data> for FetchOptionWriteMut<'data, C>
where
    C: Component,
{
    type Item = Option<&'data mut C>;
}
