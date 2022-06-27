use std::marker::PhantomData;

use crate::component::marker::Not;
use crate::component::Component;
use crate::system::fetch::Fetch;

pub struct FetchNot<C>(PhantomData<C>)
where
    C: Component;

impl<'data, C> Fetch<'data> for FetchNot<C>
where
    C: Component,
{
    type Item = Not<C>;
}
