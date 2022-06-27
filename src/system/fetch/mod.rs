pub use impls::*;

mod impls;
mod tuple;

pub trait Fetch<'data>: 'data {
    type Item: Send + Sync + 'data;
}
