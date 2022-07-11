//! Provides making [queries](crate::world::query) on the world.

pub(crate) use entities::Entities;
pub use view_mut::ViewMut;
pub use view_one::ViewOne;
pub use view_one_mut::ViewOneMut;
pub use view_shared::View;

mod entities;
mod view_mut;
mod view_one;
mod view_one_mut;
mod view_shared;
