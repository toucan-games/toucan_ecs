use crate::component::pool::ComponentPool;
use crate::{Component, Entity, RefMut, Registry};

use super::Fetch;

pub struct FetchWrite<'data, C>
where
    C: Component,
{
    pool: &'data ComponentPool<C>,
}

impl<'data, C> TryFrom<&'data Registry> for FetchWrite<'data, C>
where
    C: Component,
{
    type Error = ();

    // noinspection DuplicatedCode
    fn try_from(registry: &'data Registry) -> Result<Self, Self::Error> {
        let pool = registry.get_pool::<C>().ok_or(())?;
        Ok(Self { pool })
    }
}

impl<'data, C> Fetch<'data> for FetchWrite<'data, C>
where
    C: Component,
{
    type Item = RefMut<'data, C>;

    fn fetch(&self, entity: Entity) -> Result<Self::Item, ()> {
        self.pool.get_mut(entity).ok_or(())
    }
}
