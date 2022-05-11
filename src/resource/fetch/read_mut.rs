use std::marker::PhantomData;

use crate::resource::Resource;
use crate::world::FetchMut;

pub struct FetchReadMut<'data, R>
where
    R: Resource,
{
    _ph: PhantomData<&'data R>,
}

impl<'data, R> FetchMut<'data> for FetchReadMut<'data, R>
where
    R: Resource,
{
    type Item = &'data R;
}
