pub use read::*;
pub use write::*;

mod impls;
mod read;
mod write;

pub trait Fetch<'data>: 'data {
    type Item: Send + Sync + 'data;
}
