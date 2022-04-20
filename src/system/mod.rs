//! Provides traits and utilities for [systems][`System`].

use as_any::AsAny;

use holder::SystemHolder;
pub use schedule::{Schedule, ScheduleBuilder};

mod holder;
mod impls;
mod schedule;

/// Trait for systems in ECS.
///
/// Each system performs global actions on every [entity][`crate::Entity`]
/// that possesses a [component][`crate::component::Component`]
/// or components that match that system's query.
pub trait System<Query>: AsAny {
    /// This function is called every time you need to update the state of the world.
    fn run(&mut self, args: Query);
}
