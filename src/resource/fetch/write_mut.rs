use crate::resource::Resource;
use crate::world::{FetchError, FetchMut};
use crate::{Entity, World};

#[repr(transparent)]
pub struct FetchWriteMut<'data, R>
where
    R: Resource,
{
    resource: &'data mut R,
}

impl<'data, R> TryFrom<&'data mut World> for FetchWriteMut<'data, R>
where
    R: Resource,
{
    type Error = FetchError;

    fn try_from(world: &'data mut World) -> Result<Self, Self::Error> {
        let resource = world.resources_mut().get_mut().ok_or(FetchError)?;
        Ok(Self { resource })
    }
}

impl<'data, R> FetchMut<'data> for FetchWriteMut<'data, R>
where
    R: Resource,
{
    type Item = &'data mut R;

    fn fetch_mut(&'data mut self, _: Entity) -> Result<Self::Item, FetchError> {
        Ok(self.resource)
    }
}
