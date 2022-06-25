use std::marker::PhantomData;

use crate::component::Component;
#[cfg(feature = "resource")]
use crate::resource::{marker, Resource};

use super::Fetch;

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
}
