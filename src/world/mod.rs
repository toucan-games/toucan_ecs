//! General management of entities, their components
//! and resources (if enabled by the feature `resource`).

pub use components::{Components, ComponentsMut};
pub use entry::Entry;
pub use impls::World;
pub(crate) use world_refs::WorldRefs;
#[cfg(feature = "resource")]
pub use {
    resources::{Resources, ResourcesMut},
    split::{Split, SplitMut},
};

mod components;
mod entry;
mod impls;
#[cfg(feature = "resource")]
mod resources;
#[cfg(feature = "resource")]
mod split;
mod world_refs;

pub mod query;
pub mod view;
