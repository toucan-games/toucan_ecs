use crate::system::query::CheckedQuery;
use crate::system::{Query, System};
use crate::World;

#[repr(transparent)]
pub struct ErasedSystemHolder<'data>(Box<dyn Holdable + 'data>);

impl<'data, H> From<H> for ErasedSystemHolder<'data>
where
    H: Holdable + 'data,
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

trait Holdable {
    fn run(&mut self, world: &mut World);
}

impl<'data, S, Q> Holdable for (S, CheckedQuery<'data, Q>)
where
    S: System<'data, Q>,
    Q: Query<'data>,
{
    fn run(&mut self, _world: &mut World) {
        let system = &mut self.0;
        let args: Q = todo!();
        system.run(args)
    }
}
