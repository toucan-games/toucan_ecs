use std::marker::PhantomData;

use crate::component::Component;
use crate::world::view::ViewOneMut;
use crate::world::World;

pub struct FetchViewOneMut<C>
where
    C: Component,
{
    _ph: PhantomData<C>,
}

impl<'data, C> FetchViewOneMut<C>
where
    C: Component,
{
    pub unsafe fn fetch(world: *mut World) -> ViewOneMut<'data, C> {
        let world = &mut *world;
        let storage = world.components_mut().get_storage_mut::<C>();
        ViewOneMut::new(storage)
    }
}
