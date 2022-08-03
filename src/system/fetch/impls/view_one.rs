use std::marker::PhantomData;

use crate::component::Component;
use crate::entity::Iter;
use crate::error::FetchResult;
use crate::system::fetch::Fetch;
use crate::world::view::ViewOne;
use crate::world::WorldRefs;

#[repr(transparent)]
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

    fn fetch(_: &Iter<'data>, data: &mut WorldRefs<'data>) -> FetchResult<Self::Item> {
        let storage = data.move_storage_ref::<C>();
        let view_one = ViewOne::new(storage);
        Ok(view_one)
    }
}
