use std::marker::PhantomData;

use crate::component::Component;
use crate::system::fetch::Fetch;

pub struct FetchOptionWrite<C>
where
    C: Component,
{
    _ph: PhantomData<C>,
}

impl<'data, C> Fetch<'data> for FetchOptionWrite<C>
where
    C: Component,
{
    type Item = Option<&'data mut C>;
}
