use std::marker::PhantomData;

use crate::system::{Query, System};
use crate::World;

#[repr(transparent)]
pub struct SystemHolder(Box<dyn SystemRunnable>);

impl<R> From<R> for SystemHolder
where
    R: SystemRunnable,
{
    fn from(system_runnable: R) -> Self {
        Self(Box::new(system_runnable))
    }
}

impl SystemHolder {
    pub fn run(&mut self, world: &mut World) {
        self.0.run(world)
    }
}

trait SystemRunnable: 'static {
    fn run(&mut self, world: &mut World);
}

impl<'data, S, Q> SystemRunnable for (S, PhantomData<Q>)
where
    S: System<'data, Q>,
    Q: Query<'data> + 'static,
{
    fn run(&mut self, _world: &mut World) {
        let system = &mut self.0;
        let args = todo!();
        system.run(args)
    }
}
