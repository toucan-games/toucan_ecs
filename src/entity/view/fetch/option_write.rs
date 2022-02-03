use crate::component::pool::ComponentPool;
use crate::{Component, Entity, RefMut, Registry};

use super::Fetch;

pub struct FetchOptionWrite<'data, C>
where
    C: Component,
{
    pool: Option<&'data ComponentPool<C>>,
}

impl<'data, C> TryFrom<&'data Registry> for FetchOptionWrite<'data, C>
where
    C: Component,
{
    type Error = ();

    fn try_from(registry: &'data Registry) -> Result<Self, Self::Error> {
        let pool = registry.get_pool::<C>();
        Ok(Self { pool })
    }
}

impl<'data, C> Fetch<'data> for FetchOptionWrite<'data, C>
where
    C: Component,
{
    type Item = Option<RefMut<'data, C>>;

    fn fetch(&self, entity: Entity) -> Result<Self::Item, ()> {
        let item = self.pool.and_then(|pool| pool.get_mut(entity));
        Ok(item)
    }
}
