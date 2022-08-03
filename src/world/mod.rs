//! General management of entities, their components
//! and resources (if enabled by the feature `resource`).

pub use entry::Entry;
pub use impls::World;
pub(crate) use world_refs::WorldRefs;

mod entry;
mod impls;
mod world_refs;

pub mod query;
pub mod view;
