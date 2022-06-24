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

impl<'data, R> Fetch<'data> for FetchRead<'data, R>
where
    R: Resource,
{
    type Item = ResourceMarker<'data, R>;

    fn new(world: WorldData<'data>) -> Result<Self, FetchError> {
        let resource = world.resources().get().ok_or(FetchError)?;
        Ok(Self { resource })
    }

    fn fetch(&self, _: Entity) -> Result<Self::Item, FetchError> {
        let resource = ResourceMarker::new(self.resource);
        Ok(resource)
    }
}
