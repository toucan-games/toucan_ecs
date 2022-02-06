use crate::component::{DefaultStorage, Ref};
use crate::world::Fetch;
use crate::{Component, Entity, Registry};

pub struct FetchOptionRead<'data, C>
where
    C: Component,
{
    storage: Option<&'data DefaultStorage<C>>,
}

impl<'data, C> TryFrom<&'data Registry> for FetchOptionRead<'data, C>
where
    C: Component,
{
    type Error = ();

    fn try_from(registry: &'data Registry) -> Result<Self, Self::Error> {
        let storage = registry.get_storage::<C>();
        Ok(Self { storage })
    }
}

impl<'data, C> Fetch<'data> for FetchOptionRead<'data, C>
where
    C: Component,
{
    type Item = Option<Ref<'data, C>>;

    fn fetch(&self, entity: Entity) -> Result<Self::Item, ()> {
        let item = self.storage.and_then(|storage| storage.get(entity));
        Ok(item)
    }
}
