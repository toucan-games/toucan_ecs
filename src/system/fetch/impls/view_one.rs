use std::marker::PhantomData;

use atomicell::Ref;

use crate::component::Component;
use crate::error::FetchResult;
use crate::system::fetch::Fetch;
use crate::world::view::ViewOne;
use crate::world::World;

#[repr(transparent)]
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

    fn fetch(world: &'data World) -> FetchResult<Self::Item> {
        let storage = world.components().get_storage_guarded::<C>();
        let storage = storage.map(Ref::leak);
        let view_one = ViewOne::new(storage);
        Ok(view_one)
    }
}
