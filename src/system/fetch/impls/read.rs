use std::marker::PhantomData;

use crate::component::Component;
#[cfg(feature = "resource")]
use crate::resource::{marker, Resource};
use crate::system::fetch::Fetch;
use crate::World;

pub struct FetchRead<C>
where
    C: Component,
{
    _ph: PhantomData<C>,
}

impl<'data, C> Fetch<'data> for FetchRead<C>
where
    C: Component,
{
    type Item = &'data C;

    unsafe fn fetch(_world: &'data mut World) -> Self::Item {
        todo!()
    }
}

#[cfg(feature = "resource")]
pub struct FetchResourceRead<R>
where
    R: Resource,
{
    _ph: PhantomData<R>,
}

#[cfg(feature = "resource")]
impl<'data, R> Fetch<'data> for FetchResourceRead<R>
where
    R: Resource,
{
    type Item = marker::Resource<'data, R>;

    unsafe fn fetch(_world: &'data mut World) -> Self::Item {
        todo!()
    }
}
