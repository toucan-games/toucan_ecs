use std::marker::PhantomData;

use crate::resource::Resource;
use crate::world::FetchMut;

pub struct FetchWriteMut<'data, R>
where
    R: Resource,
{
    _ph: PhantomData<&'data mut R>,
}

impl<'data, R> FetchMut<'data> for FetchWriteMut<'data, R>
where
    R: Resource,
{
    type Item = &'data mut R;
}
