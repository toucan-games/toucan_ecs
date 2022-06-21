//! Provides traits and utilities for [systems][`System`].

use fetch::Fetch;
use query::Query;
pub use schedule::{Schedule, ScheduleBuilder};

mod fetch;
mod impls;
mod query;
mod schedule;

/// Trait for systems in ECS.
///
/// Each system performs global actions on every [entity][`crate::Entity`]
/// that possesses a [component][`crate::component::Component`]
/// or components that match that system's query.
pub trait System<'data, Q>: 'data
where
    Q: Query<'data>,
{
    /// This function is called every time you need to update the state of the world.
    fn run(&mut self, args: Q);
}
