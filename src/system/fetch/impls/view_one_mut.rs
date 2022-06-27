use std::marker::PhantomData;

use crate::component::{Component, ViewOneMut};
use crate::system::fetch::Fetch;

pub struct FetchViewOneMut<C>
where
    C: Component,
{
    _ph: PhantomData<C>,
}

impl<'data, C> Fetch<'data> for FetchViewOneMut<C>
where
    C: Component,
{
    type Item = ViewOneMut<'data, C>;
}
