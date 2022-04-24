use std::marker::PhantomData;

use holder::SystemHolder;

use crate::World;

use super::{Query, System};

mod holder;

/// A schedule of systems for execution.
///
/// This struct is used to run systems one by one in expected order
/// which is defined by sequential calls of [`ScheduleBuilder::system`] function.
#[repr(transparent)]
pub struct Schedule {
    systems: Vec<SystemHolder>,
}

impl Schedule {
    /// Creates a new [schedule][`Schedule`] builder.
    pub fn builder() -> ScheduleBuilder {
        ScheduleBuilder::new()
    }

    /// Executes all the systems inside of schedule.
    pub fn run(&mut self, world: &mut World) {
        self.systems.iter_mut().for_each(|system| system.run(world))
    }
}

/// A builder for [`Schedule`] struct.
pub struct ScheduleBuilder {
    systems: Vec<SystemHolder>,
}

impl ScheduleBuilder {
    fn new() -> Self {
        Self {
            systems: Vec::new(),
        }
    }

    /// Adds a system to the [schedule][`Schedule`].
    pub fn system<'data, S, Q>(mut self, system: S) -> Self
    where
        S: System<'data, Q>,
        Q: Query<'data> + 'static,
    {
        self.systems.push((system, PhantomData).into());
        self
    }

    /// Finalizes the builder into a [schedule][`Schedule`].
    pub fn build(self) -> Schedule {
        let systems = self.systems;
        Schedule { systems }
    }
}
