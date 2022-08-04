//! Prelude module contains the most commonly used crate items.

#[doc(hidden)]
pub use crate::{
    component::storage::{DefaultStorage, DynIter, DynIterMut, Storage},
    component::Component,
    entity::{Entity, EntityBuilder},
    system::{Schedule, ScheduleBuilder, System},
    world::view::{View, ViewMut, ViewOne, ViewOneMut},
    world::{Components, ComponentsMut, Entry, World},
};
#[doc(hidden)]
#[cfg(feature = "resource")]
pub use crate::{
    resource::Resource,
    world::{Resources, ResourcesMut, Split, SplitMut},
};
