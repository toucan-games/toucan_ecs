//! Prelude module contains the most commonly used crate items.

pub use crate::{
    component::storage::{DefaultStorage, DynIter, DynIterMut, Storage},
    component::Component,
    entity::{Entity, EntityBuilder},
    marker::*,
    system::{Schedule, ScheduleBuilder, System},
    world::view::{View, ViewMut, ViewOne, ViewOneMut},
    world::{Components, ComponentsMut, Entry, World},
};
#[cfg(feature = "resource")]
pub use crate::{
    resource::Resource,
    world::{Resources, ResourcesMut, Split, SplitMut},
};
