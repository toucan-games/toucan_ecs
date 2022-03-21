//! General management of entities, their components
//! and resources (if enabled by the feature `resource`).

pub(crate) use hash::TypeIdHasher;
pub use impls::World;

mod hash;
mod impls;
