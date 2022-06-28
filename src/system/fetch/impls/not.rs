use std::marker::PhantomData;

use crate::component::marker::Not;
use crate::component::Component;
use crate::error::FetchResult;
use crate::system::fetch::Fetch;
use crate::World;

pub struct FetchNot<C>(PhantomData<C>)
where
    C: Component;

impl<'data, C> Fetch<'data> for FetchNot<C>
where
    C: Component,
{
    type Item = Not<C>;

    unsafe fn fetch(_world: *mut World) -> FetchResult<Self::Item> {
        todo!()
    }
}
