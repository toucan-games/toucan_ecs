use crate::Entity;
use crate::resource::{marker::Resource as ResourceMarker, Resource};
use crate::world::{Fetch, FetchError, WorldData};

#[repr(transparent)]
pub struct FetchRead<'data, R>
where
    R: Resource,
{
    resource: &'data R,
}

impl<'data, R> TryFrom<WorldData<'data>> for FetchRead<'data, R>
where
    R: Resource,
{
    type Error = FetchError;

    fn try_from(world: WorldData<'data>) -> Result<Self, Self::Error> {
        let resource = world.resources().get().ok_or(FetchError)?;
        Ok(Self { resource })
    }
}

impl<'data, R> Fetch<'data> for FetchRead<'data, R>
where
    R: Resource,
{
    type Item = ResourceMarker<'data, R>;

    fn fetch(&self, _: Entity) -> Result<Self::Item, FetchError> {
        let resource = ResourceMarker::new(self.resource);
        Ok(resource)
    }
}
