use crate::Entity;
use crate::resource::{marker::Resource as ResourceMarker, Resource};
use crate::world::{FetchError, FetchMut, WorldDataMut};

#[repr(transparent)]
pub struct FetchReadMut<'data, R>
where
    R: Resource,
{
    resource: &'data R,
}

impl<'data, R> FetchMut<'data> for FetchReadMut<'data, R>
where
    R: Resource,
{
    type Item = ResourceMarker<'data, R>;

    unsafe fn new(world: WorldDataMut<'data>) -> Result<Self, FetchError> {
        // SAFETY: must be checked by the caller.
        let resource = world.resources().get().ok_or(FetchError)?;
        Ok(Self { resource })
    }

    fn fetch_mut(&mut self, _: Entity) -> Result<Self::Item, FetchError> {
        let resource = ResourceMarker::new(self.resource);
        Ok(resource)
    }
}
