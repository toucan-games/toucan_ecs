//! Utilities for [foreach systems](ForeachSystem) - special kind
//! of [systems](crate::system::System).

pub(crate) use checked::CheckedQuery;
pub use convert::FromForeachSystem;
pub(crate) use holder::ForeachHolder;
pub use query::Query;

mod checked;
mod convert;
mod fetch;
mod holder;
mod impls;
mod query;
mod tuple;

/// Trait for special foreach systems in ECS.
///
/// Each system performs global actions on every [entity](crate::entity::Entity)
/// that possesses a [component](crate::component::Component)
/// or components that match that system's query.
pub trait ForeachSystem<'data, Q>: 'data
where
    Q: Query<'data>,
{
    /// This function is called every time you need to update the state of the world.
    fn run(&mut self, args: Q);
}
