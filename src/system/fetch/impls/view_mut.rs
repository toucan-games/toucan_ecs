use std::marker::PhantomData;

use crate::entity::Iter;
use crate::error::FetchResult;
use crate::system::fetch::Fetch;
use crate::world::query::QueryMut;
use crate::world::view::ViewMut;
use crate::world::WorldRefs;

#[repr(transparent)]
pub struct FetchViewMut<'data, Q>
where
    Q: QueryMut<'data>,
{
    _ph: PhantomData<&'data Q>,
}

impl<'data, Q> Fetch<'data> for FetchViewMut<'data, Q>
where
    Q: QueryMut<'data>,
{
    type Item = ViewMut<'data, Q>;

    fn fetch(entities: &Iter<'data>, data: &mut WorldRefs<'data>) -> FetchResult<Self::Item> {
        let view_mut = ViewMut::new(entities.clone(), data);
        Ok(view_mut)
    }
}
