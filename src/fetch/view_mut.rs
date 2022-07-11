use std::marker::PhantomData;

use crate::world::query::QueryMut;
use crate::world::view::ViewMut;
use crate::World;

pub struct FetchViewMut<'data, Q>
where
    Q: QueryMut<'data>,
{
    _ph: PhantomData<&'data Q>,
}

impl<'data, Q> FetchViewMut<'data, Q>
where
    Q: QueryMut<'data>,
{
    pub unsafe fn fetch(world: *mut World) -> ViewMut<'data, Q> {
        let world = &mut *world;
        ViewMut::new(world)
    }
}
