use std::marker::PhantomData;

use crate::component::Component;
use crate::entity::Iter;
use crate::error::FetchResult;
use crate::system::fetch::Fetch;
use crate::world::view::ViewOneMut;
use crate::world::WorldRefs;

#[repr(transparent)]
pub struct FetchViewOneMut<C>
where
    C: Component,
{
    _ph: PhantomData<C>,
}

impl<'data, C> Fetch<'data> for FetchViewOneMut<C>
where
    C: Component,
{
    type Item = ViewOneMut<'data, C>;

    fn fetch(_: &Iter<'data>, data: &mut WorldRefs<'data>) -> FetchResult<Self::Item> {
        let storage = data.move_storage_ref_mut::<C>();
        let view_one_mut = ViewOneMut::new(storage);
        Ok(view_one_mut)
    }
}
