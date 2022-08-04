//! Special marker types for views and systems.

pub use component::Not;
#[cfg(feature = "resource")]
pub use resource::{Resource, ResourceMut};

mod component;
#[cfg(feature = "resource")]
mod resource;
