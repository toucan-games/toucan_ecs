//! General management of entities, their components
//! and resources (if enabled by the feature `resource`).

pub use entry::Entry;
pub(crate) use fetch::{Fetch, FetchError, FetchMut};
pub use impls::World;
pub(crate) use impls::{WorldData, WorldDataMut};

mod entry;
mod fetch;
mod impls;

pub mod query;
pub mod view;
