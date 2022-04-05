//! Provides traits and utilities for [systems][`System`].

mod impls;

/// Trait for systems in ECS.
///
/// Each system performs global actions on every [entity][`crate::Entity`]
/// that possesses a [component][`crate::component::Component`]
/// or components that match that system's query.
pub trait System<Query> {
    /// This function is called every time you need to update the state of the world.
    fn run(&mut self, args: Query);
}
