use std::marker::PhantomData;

use crate::system::foreach::{ForeachHolder, Query};
use crate::World;

#[repr(transparent)]
pub struct FetchForeachHolder<'data, Q>
where
    Q: Query<'data>,
{
    _ph: PhantomData<&'data Q>,
}

impl<'data, Q> FetchForeachHolder<'data, Q>
where
    Q: Query<'data>,
{
    pub unsafe fn fetch(world: *mut World) -> ForeachHolder<'data, Q> {
        let world = &mut *world;
        ForeachHolder::new(world)
    }
}
