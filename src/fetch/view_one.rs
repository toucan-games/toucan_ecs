use std::marker::PhantomData;

use crate::component::Component;
use crate::world::view::ViewOne;
use crate::world::World;

pub struct FetchViewOne<C>
where
    C: Component,
{
    _ph: PhantomData<C>,
}

impl<'data, C> FetchViewOne<C>
where
    C: Component,
{
    pub unsafe fn fetch(world: *mut World) -> ViewOne<'data, C> {
        let world = &*world;
        let storage = world.components().get_storage::<C>();
        ViewOne::new(storage)
    }
}
