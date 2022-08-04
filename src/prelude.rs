//! Prelude module contains all of the most commonly used crate items.

#[doc(hidden)]
pub use crate::{
    world::{World, Entry},
    world::view::{View, ViewMut, ViewOne, ViewOneMut},
    entity::{Entity, EntityBuilder},
    component::Component,
    component::storage::{Storage, DefaultStorage, DynIter, DynIterMut},
    system::{System, Schedule, ScheduleBuilder},
};

#[doc(hidden)]
#[cfg(feature = "resource")]
pub use crate::resource::marker::{Resource, ResourceMut};
