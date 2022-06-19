use crate::resource::Resource;
use crate::world::{Fetch, FetchError};
use crate::{Entity, World};

#[repr(transparent)]
pub struct FetchRead<'data, R>
where
    R: Resource,
{
    resource: &'data R,
}

impl<'data, R> TryFrom<&'data World> for FetchRead<'data, R>
where
    R: Resource,
{
    type Error = FetchError;

    fn try_from(world: &'data World) -> Result<Self, Self::Error> {
        let resource = world.resources().get().ok_or(FetchError)?;
        Ok(Self { resource })
    }
}

impl<'data, R> Fetch<'data> for FetchRead<'data, R>
where
    R: Resource,
{
    type Item = &'data R;

    fn fetch(&self, _: Entity) -> Result<Self::Item, FetchError> {
        Ok(self.resource)
    }
}
