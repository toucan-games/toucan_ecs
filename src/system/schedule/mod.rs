use std::marker::PhantomData;

use holder::ErasedSystemHolder;

use crate::system::foreach::{ForeachSystem, FromForeachSystem, Query as ForeachQuery};
use crate::world::World;

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

    /// Executes all the systems inside of schedule
    /// in the order of their addition.
    pub fn run(&mut self, world: &mut World) {
        for system in self.systems.iter_mut() {
            system.run(world);
            world.components_mut().undo_leak();
            world.resources_mut().undo_leak();
        }
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

    /// Adds the system to the [schedule](Schedule).
    ///
    /// # Panics
    ///
    /// This function will panic if provided query does not satisfies
    /// the first rule of references described in
    /// **References and Borrowing** section of [**Rust Book**][rust_book]:
    ///
    /// > - *At any given time, you can have either **one** mutable reference
    /// or **any** number of immutable references.*
    ///
    /// [rust_book]: https://doc.rust-lang.org/book/ch04-02-references-and-borrowing.html#the-rules-of-references
    pub fn system<S, Q>(mut self, system: S) -> Self
    where
        S: System<'data, Q>,
        Q: Query<'data>,
    {
        let erased = (system, PhantomData).into();
        self.systems.push(erased);
        self
    }

    /// Adds the foreach system to the [schedule](Schedule).
    ///
    /// # Panics
    ///
    /// This function will panic if provided query does not satisfies
    /// the first rule of references described in
    /// **References and Borrowing** section of [**Rust Book**][rust_book]:
    ///
    /// > - *At any given time, you can have either **one** mutable reference
    /// or **any** number of immutable references.*
    ///
    /// [rust_book]: https://doc.rust-lang.org/book/ch04-02-references-and-borrowing.html#the-rules-of-references
    pub fn foreach_system<S, Q>(self, system: S) -> Self
    where
        S: ForeachSystem<'data, Q>,
        Q: ForeachQuery<'data>,
    {
        let system = FromForeachSystem::from(system);
        self.system(system)
    }

    /// Finalizes the builder into a [schedule](Schedule).
    pub fn build(self) -> Schedule<'data> {
        let systems = self.systems;
        Schedule { systems }
    }
}
