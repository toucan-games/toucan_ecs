use std::marker::PhantomData;

use crate::component::Component;
use crate::error::FetchResult;
use crate::system::fetch::Fetch;
use crate::World;

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

    unsafe fn fetch(_world: *mut World) -> FetchResult<Self::Item> {
        todo!()
    }
}
