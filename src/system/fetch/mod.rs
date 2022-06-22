pub use option_read::*;
pub use option_write::*;
pub use read::*;
pub use view::*;
pub use view_mut::*;
pub use view_one::*;
pub use view_one_mut::*;
pub use write::*;

mod impls;
mod option_read;
mod option_write;
mod read;
mod view;
mod view_mut;
mod view_one;
mod view_one_mut;
mod write;

pub trait Fetch<'data>: 'data {
    type Item: Send + Sync + 'data;
}
