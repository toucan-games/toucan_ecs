use std::mem::transmute;

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
    pub fn run(&mut self, world: &mut World) {
        self.0.run(world)
    }
}

trait Holdable<'data>: 'data {
    fn run(&mut self, world: &mut World);
}

impl<'data, S, Q> Holdable<'data> for (S, CheckedQuery<'data, Q>)
where
    S: System<'data, Q>,
    Q: Query<'data>,
{
    // noinspection RsUnnecessaryQualifications
    fn run(&mut self, world: &mut World) {
        // SAFETY: `world` contains data which is alive for `'data` lifetime
        let world: &'data mut World = unsafe { transmute(world) };
        let (system, _) = self;

        let (entities, mut data) = world.split_refs_system_mut();
        let entities = entities.iter();
        let args = Q::Fetch::fetch(&entities, &mut data);
        if let Ok(args) = args {
            let args = args.into();
            system.run(args)
        }
    }
}
