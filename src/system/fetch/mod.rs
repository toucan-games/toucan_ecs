pub use option_read::*;
pub use option_write::*;
pub use read::*;
pub use write::*;

mod impls;
mod option_read;
mod option_write;
mod read;
mod write;

pub trait Fetch<'data>: 'data {
    type Item: Send + Sync + 'data;
}
