//! General management of entities, their components
//! and resources (if enabled by the feature `resource`).

pub use components::{Components, ComponentsMut};
pub use entry::Entry;
pub use impls::World;
pub(crate) use world_refs::WorldRefs;
#[cfg(feature = "resource")]
pub use {
    impls::{Split, SplitMut},
    resources::{Resources, ResourcesMut},
};

mod components;
mod entry;
mod impls;
#[cfg(feature = "resource")]
mod resources;
mod world_refs;

pub mod query;
pub mod view;
