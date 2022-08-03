pub use impls::*;

use crate::component::ComponentTypeId;
use crate::entity::{Entity, Iter};
use crate::error::FetchResult;
use crate::world::WorldRefs;

mod impls;
mod tuple;

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct FetchData {
    type_id: ComponentTypeId,
    len: usize,
}

impl FetchData {
    pub fn new(type_id: ComponentTypeId, len: usize) -> Self {
        Self { type_id, len }
    }

    pub fn type_id(&self) -> ComponentTypeId {
        self.type_id
    }

    pub fn into_type_id(self) -> ComponentTypeId {
        self.type_id
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn into_len(self) -> usize {
        self.len
    }
}

pub fn find_optimal<'data, F>(data: &WorldRefs<'data>) -> Option<FetchData>
where
    F: Fetch<'data>,
{
    let mut fetch_data = Vec::new();
    F::push_fetch_data(data, &mut fetch_data);
    fetch_data.sort_unstable_by_key(FetchData::len);
    fetch_data.into_iter().next()
}

pub enum FetchStrategy<'data> {
    All(&'data mut Iter<'data>),
    Optimized,
}

pub trait Fetch<'data>: Sized + Send + Sync + 'data {
    type Item: Send + Sync + 'data;

    fn push_fetch_data(data: &WorldRefs<'data>, fetch_data: &mut Vec<FetchData>);

    fn new(data: &mut WorldRefs<'data>, optimal: Option<ComponentTypeId>) -> FetchResult<Self>;

    fn is_iter(&self) -> bool;

    fn fetch_entity(&'data mut self, entity: Entity) -> FetchResult<Self::Item>;

    fn fetch_iter(
        &'data mut self,
        strategy: FetchStrategy<'data>,
    ) -> FetchResult<Option<(Entity, Self::Item)>>;
}
