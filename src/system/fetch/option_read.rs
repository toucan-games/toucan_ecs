use std::marker::PhantomData;

use crate::component::Component;

use super::Fetch;

pub struct FetchOptionRead<C>
where
    C: Component,
{
    _ph: PhantomData<C>,
}

impl<'data, C> Fetch<'data> for FetchOptionRead<C>
where
    C: Component,
{
    type Item = Option<&'data C>;
}
