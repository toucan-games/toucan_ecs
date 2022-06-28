use std::marker::PhantomData;

use crate::component::Component;
use crate::error::FetchResult;
use crate::system::fetch::Fetch;
use crate::world::view::ViewOne;
use crate::World;

pub struct FetchViewOne<C>
where
    C: Component,
{
    _ph: PhantomData<C>,
}

impl<'data, C> Fetch<'data> for FetchViewOne<C>
where
    C: Component,
{
    type Item = ViewOne<'data, C>;

    unsafe fn fetch(world: *mut World) -> FetchResult<Self::Item> {
        let world = &*world;
        let storage = world.components().get_storage();
        let view_one = ViewOne::new(storage);
        Ok(view_one)
    }
}
