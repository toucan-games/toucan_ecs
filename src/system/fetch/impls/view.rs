use std::marker::PhantomData;

use crate::entity::Iter;
use crate::error::FetchResult;
use crate::system::fetch::Fetch;
use crate::world::query::Query;
use crate::world::view::View;
use crate::world::WorldRefs;

#[repr(transparent)]
pub struct FetchView<'data, Q>
where
    Q: Query<'data>,
{
    _ph: PhantomData<&'data Q>,
}

impl<'data, Q> Fetch<'data> for FetchView<'data, Q>
where
    Q: Query<'data>,
{
    type Item = View<'data, Q>;

    fn fetch(entities: &Iter<'data>, data: &mut WorldRefs<'data>) -> FetchResult<Self::Item> {
        let view = View::new(entities.clone(), data);
        Ok(view)
    }
}
