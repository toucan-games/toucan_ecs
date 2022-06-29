use crate::system::fetch::Fetch;
use crate::system::query::CheckedQuery;
use crate::system::{Query, System};
use crate::world::World;

#[repr(transparent)]
pub struct ErasedSystemHolder<'data>(Box<dyn Holdable<'data>>);

impl<'data, H> From<H> for ErasedSystemHolder<'data>
where
    H: Holdable<'data>,
{
    fn from(holdable: H) -> Self {
        Self(Box::new(holdable))
    }
}

impl<'data> ErasedSystemHolder<'data> {
    pub fn run(&mut self, world: *mut World) {
        // SAFETY: provided pointer is valid
        let world = unsafe { &mut *world };
        self.0.run(world)
    }
}

trait Holdable<'data>: 'data {
    fn run(&mut self, world: &'data mut World);
}

impl<'data, S, Q> Holdable<'data> for (S, CheckedQuery<'data, Q>)
where
    S: System<'data, Q>,
    Q: Query<'data>,
{
    // noinspection RsUnnecessaryQualifications
    fn run(&mut self, world: &'data mut World) {
        let system = &mut self.0;
        // SAFETY: was checked because of CheckedQuery struct was constructed
        let args = unsafe { Q::Fetch::fetch(world) };
        if let Ok(args) = args {
            let args = args.into();
            system.run(args)
        }
    }
}
