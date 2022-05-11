use std::marker::PhantomData;

use crate::component::marker::Not;
use crate::component::Component;
use crate::world::FetchMut;

pub struct FetchNotMut<'data, C>
where
    C: Component,
{
    _ph: PhantomData<&'data C>,
}

impl<'data, C> FetchMut<'data> for FetchNotMut<'data, C>
where
    C: Component,
{
    type Item = Not<'data, C>;
}
