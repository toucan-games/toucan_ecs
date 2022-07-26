use std::marker::PhantomData;
use std::mem::transmute;

use crate::system::fetch::Fetch;
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

impl<'data, S, Q> Holdable<'data> for (S, PhantomData<Q>)
where
    S: System<'data, Q>,
    Q: Query<'data>,
{
    // noinspection RsUnnecessaryQualifications
    fn run(&mut self, world: &mut World) {
        // SAFETY: `world` contains data which is alive for `'data` lifetime
        let world = unsafe { transmute(world) };
        let system = &mut self.0;
        let args = Q::Fetch::fetch(world);
        if let Ok(args) = args {
            let args = args.into();
            system.run(args)
        }
    }
}
