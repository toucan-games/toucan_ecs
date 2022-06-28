use std::marker::PhantomData;

use crate::component::Component;
use crate::error::FetchResult;
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

    unsafe fn fetch(world: *mut World) -> FetchResult<Self::Item> {
        let world = &mut *world;
        let storage = world.components_mut().get_storage_mut();
        let view_one_mut = ViewOneMut::new(storage);
        Ok(view_one_mut)
    }
}
