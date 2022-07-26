use std::marker::PhantomData;

use atomicell::RefMut;

use crate::component::Component;
use crate::error::FetchResult;
use crate::system::fetch::Fetch;
use crate::world::view::ViewOneMut;
use crate::world::World;

#[repr(transparent)]
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

    fn fetch(world: &'data World) -> FetchResult<Self::Item> {
        let storage = world.components().get_storage_mut_guarded::<C>();
        let storage = storage.map(RefMut::leak);
        let view_one_mut = ViewOneMut::new(storage);
        Ok(view_one_mut)
    }
}
