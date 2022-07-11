use std::marker::PhantomData;

use crate::world::query::Query;
use crate::world::view::View;
use crate::world::World;

pub struct FetchView<'data, Q>
where
    Q: Query<'data>,
{
    _ph: PhantomData<&'data Q>,
}

impl<'data, Q> FetchView<'data, Q>
where
    Q: Query<'data>,
{
    pub unsafe fn fetch(world: *mut World) -> View<'data, Q> {
        let world = &*world;
        View::new(world)
    }
}
