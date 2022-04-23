pub use read::*;
pub use write::*;

mod impls;
mod read;
mod write;

pub trait Fetch<'data> {
    type Item: Send + Sync + 'data;
}
