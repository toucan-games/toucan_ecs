//! General management of entities, their components
//! and resources (if enabled by the feature `resource`).

pub(crate) use fetch::Fetch;
pub(crate) use fetch::FetchError;
pub(crate) use hash::TypeIdHasher;
pub use impls::World;
pub use view::View;
pub(crate) use viewable::{Viewable, ViewableItem};

mod fetch;
mod hash;
mod impls;
mod view;
mod viewable;
