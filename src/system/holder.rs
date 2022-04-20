use crate::system::System;
use crate::World;

#[repr(transparent)]
pub struct SystemHolder(Box<dyn Runnable>);

impl<T> From<T> for SystemHolder
where
    T: System<()>,
{
    fn from(system: T) -> Self {
        Self(Box::new(system))
    }
}

impl SystemHolder {
    pub fn run(&mut self, world: &mut World) {
        self.0.run(world)
    }
}

trait Runnable {
    fn run(&mut self, world: &mut World);
}

impl<T> Runnable for T
where
    T: System<()>,
{
    fn run(&mut self, _: &mut World) {
        <Self as System<()>>::run(self, ())
    }
}
