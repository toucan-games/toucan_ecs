use std::marker::PhantomData;

use crate::component::Component;
use crate::world::FetchMut;

pub struct FetchOptionReadMut<'data, C>
where
    C: Component,
{
    _ph: PhantomData<Option<&'data C>>,
}

impl<'data, C> FetchMut<'data> for FetchOptionReadMut<'data, C>
where
    C: Component,
{
    type Item = Option<&'data C>;
}
