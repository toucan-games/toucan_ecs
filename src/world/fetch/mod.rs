pub use error::FetchError;

use crate::Entity;

use super::{WorldData, WorldDataMut};

mod error;
mod tuple;

pub trait Fetch<'data>: TryFrom<WorldData<'data>, Error = FetchError> + 'data {
    type Item: Send + Sync + 'data;

    fn fetch(&self, entity: Entity) -> Result<Self::Item, FetchError>;
}

pub trait FetchMut<'data>: TryFrom<WorldDataMut<'data>, Error = FetchError> + 'data {
    type Item: Send + Sync + 'data;

    /// # Safety
    ///
    /// This function should be called if and only if mutability soundness was checked
    /// by [`check_soundness`][`super::query::check_soundness`] function.
    unsafe fn fetch_mut(&'data mut self, entity: Entity) -> Result<Self::Item, FetchError>;
}
