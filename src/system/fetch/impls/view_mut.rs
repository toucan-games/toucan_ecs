use std::marker::PhantomData;

use crate::error::FetchResult;
use crate::system::fetch::Fetch;
use crate::world::query::QueryMut;
use crate::world::view::ViewMut;
use crate::world::World;

#[repr(transparent)]
pub struct FetchViewMut<'data, Q>
where
    Q: QueryMut<'data>,
{
    _ph: PhantomData<&'data Q>,
}

impl<'data, Q> Fetch<'data> for FetchViewMut<'data, Q>
where
    Q: QueryMut<'data>,
{
    type Item = ViewMut<'data, Q>;

    unsafe fn fetch(world: *mut World) -> FetchResult<Self::Item> {
        let world = &mut *world;
        Ok(ViewMut::new(world, false))
    }
}
