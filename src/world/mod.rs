//! General management of entities, their components
//! and resources (if enabled by the feature `resource`).

pub(crate) use fetch::{Fetch, FetchError};
pub(crate) use hash::TypeIdHasher;
pub use impls::World;
pub(crate) use query::{Query, QueryItem};
pub use view::View;

mod fetch;
mod hash;
mod impls;
mod query;
mod view;
