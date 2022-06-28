use std::marker::PhantomData;

use crate::component::Component;
use crate::system::fetch::Fetch;
use crate::World;

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

    unsafe fn fetch(_world: &'data mut World) -> Self::Item {
        todo!()
    }
}
