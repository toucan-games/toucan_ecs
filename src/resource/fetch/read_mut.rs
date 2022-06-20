use crate::resource::Resource;
use crate::world::{FetchError, FetchMut, WorldDataMut};
use crate::Entity;

#[repr(transparent)]
pub struct FetchReadMut<'data, R>
where
    R: Resource,
{
    resource: &'data R,
}

impl<'data, R> TryFrom<WorldDataMut<'data>> for FetchReadMut<'data, R>
where
    R: Resource,
{
    type Error = FetchError;

    fn try_from(world: WorldDataMut<'data>) -> Result<Self, Self::Error> {
        let resource = world.resources().get().ok_or(FetchError)?;
        Ok(Self { resource })
    }
}

impl<'data, R> FetchMut<'data> for FetchReadMut<'data, R>
where
    R: Resource,
{
    type Item = &'data R;

    fn fetch_mut(&mut self, _: Entity) -> Result<Self::Item, FetchError> {
        Ok(self.resource)
    }
}
