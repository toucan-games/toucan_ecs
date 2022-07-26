use std::marker::PhantomData;

use crate::error::FetchResult;
use crate::system::fetch::Fetch;
use crate::world::query::Query;
use crate::world::view::View;
use crate::world::World;

#[repr(transparent)]
pub struct FetchView<'data, Q>
where
    Q: Query<'data>,
{
    _ph: PhantomData<&'data Q>,
}

impl<'data, Q> Fetch<'data> for FetchView<'data, Q>
where
    Q: Query<'data>,
{
    type Item = View<'data, Q>;

    unsafe fn fetch(world: *mut World) -> FetchResult<Self::Item> {
        let world = &*world;
        Ok(View::new(world))
    }
}
