use std::marker::PhantomData;

use crate::system::{Query, System};
use crate::World;

#[repr(transparent)]
pub struct SystemHolder(Box<dyn Runnable>);

impl<R> From<R> for SystemHolder
where
    R: Runnable,
{
    fn from(runnable: R) -> Self {
        Self(Box::new(runnable))
    }
}

impl SystemHolder {
    pub fn run(&mut self, world: &mut World) {
        self.0.run(world)
    }
}

trait Runnable: 'static {
    fn run(&mut self, world: &mut World);
}

impl<'data, S, Q> Runnable for (S, PhantomData<Q>)
where
    S: System<'data, Q>,
    Q: Query<'data> + 'static,
{
    fn run(&mut self, _: &mut World) {
        let args = todo!();
        self.0.run(args)
    }
}
