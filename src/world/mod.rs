//! General management of entities, their components
//! and resources (if enabled by the feature `resource`).

pub use entry::Entry;
pub use impls::World;
pub(crate) use impls::WorldData;

mod entry;
mod impls;

pub mod query;
pub mod view;
