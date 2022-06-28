use std::marker::PhantomData;

use crate::component::Component;
use crate::system::fetch::Fetch;
use crate::world::view::ViewOne;

pub struct FetchViewOne<C>
where
    C: Component,
{
    _ph: PhantomData<C>,
}

impl<'data, C> Fetch<'data> for FetchViewOne<C>
where
    C: Component,
{
    type Item = ViewOne<'data, C>;
}
