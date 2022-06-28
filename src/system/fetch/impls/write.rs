use std::marker::PhantomData;

use crate::component::Component;
#[cfg(feature = "resource")]
use crate::resource::{marker, Resource};
use crate::system::fetch::Fetch;
use crate::World;

pub struct FetchWrite<C>
where
    C: Component,
{
    _ph: PhantomData<C>,
}

impl<'data, C> Fetch<'data> for FetchWrite<C>
where
    C: Component,
{
    type Item = &'data mut C;

    unsafe fn fetch(_world: &'data mut World) -> Self::Item {
        todo!()
    }
}

#[cfg(feature = "resource")]
pub struct FetchResourceWrite<R>
where
    R: Resource,
{
    _ph: PhantomData<R>,
}

#[cfg(feature = "resource")]
impl<'data, R> Fetch<'data> for FetchResourceWrite<R>
where
    R: Resource,
{
    type Item = marker::ResourceMut<'data, R>;

    unsafe fn fetch(_world: &'data mut World) -> Self::Item {
        todo!()
    }
}
