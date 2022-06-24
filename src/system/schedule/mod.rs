use std::marker::PhantomData;

use holder::ErasedSystemHolder;

use crate::World;

use super::{Query, System};

mod holder;

/// A schedule of systems for execution.
///
/// This struct is used to run systems one by one in expected order
/// which is defined by sequential calls of [`ScheduleBuilder::system`] function.
#[repr(transparent)]
pub struct Schedule<'data> {
    systems: Vec<ErasedSystemHolder<'data>>,
}

impl<'data> Schedule<'data> {
    /// Creates a new [schedule](Schedule) builder.
    pub fn builder() -> ScheduleBuilder<'data> {
        ScheduleBuilder::new()
    }

    /// Executes all the systems inside of schedule.
    pub fn run(&mut self, world: &'data mut World) {
        self.systems.iter_mut().for_each(|system| system.run(world))
    }
}

/// A builder for [`Schedule`] struct.
pub struct ScheduleBuilder<'data> {
    systems: Vec<ErasedSystemHolder<'data>>,
}

impl<'data> ScheduleBuilder<'data> {
    fn new() -> Self {
        Self {
            systems: Vec::new(),
        }
    }

    /// Adds a system to the [schedule](Schedule).
    pub fn system<S, Q>(mut self, system: S) -> Self
    where
        S: System<'data, Q>,
        Q: Query<'data>,
    {
        let erased = (system, PhantomData).into();
        self.systems.push(erased);
        self
    }

    /// Finalizes the builder into a [schedule](Schedule).
    pub fn build(self) -> Schedule<'data> {
        let systems = self.systems;
        Schedule { systems }
    }
}
