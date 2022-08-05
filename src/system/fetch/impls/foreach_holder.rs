use std::marker::PhantomData;

use crate::entity::Iter;
use crate::error::FetchResult;
use crate::system::fetch::Fetch;
use crate::system::foreach::{ForeachHolder, Query};
use crate::world::WorldRefs;

#[repr(transparent)]
pub struct FetchForeachHolder<'data, Q>
where
    Q: Query<'data>,
{
    _ph: PhantomData<&'data Q>,
}

impl<'data, Q> Fetch<'data> for FetchForeachHolder<'data, Q>
where
    Q: Query<'data>,
{
    type Item = ForeachHolder<'data, Q>;

    fn fetch(entities: &Iter<'data>, data: &mut WorldRefs<'data>) -> FetchResult<Self::Item> {
        let entities = Some(entities.clone());
        let foreach_holder = ForeachHolder::new(entities, data);
        Ok(foreach_holder)
    }
}
