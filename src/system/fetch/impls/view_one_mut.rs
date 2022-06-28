use std::marker::PhantomData;

use crate::component::Component;
use crate::system::fetch::Fetch;
use crate::world::view::ViewOneMut;
use crate::World;

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

    unsafe fn fetch(_world: &'data mut World) -> Self::Item {
        todo!()
    }
}
