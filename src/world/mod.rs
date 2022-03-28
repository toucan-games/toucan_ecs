//! General management of entities, their components
//! and resources (if enabled by the feature `resource`).

pub(crate) use fetch::{Fetch, FetchError};
pub(crate) use hash::TypeIdHasher;
pub use impls::World;
pub(crate) use query::{Query, QueryItem, QueryShared};
pub use view::{View, ViewMut};

mod fetch;
mod hash;
mod impls;
mod query;
mod view;
