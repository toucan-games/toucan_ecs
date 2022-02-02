use super::fetch::{Fetch, FetchRead, FetchWrite};

mod impls;
mod tuple;

pub type ViewableItem<'data, V> = <<V as Viewable<'data>>::Fetch as Fetch<'data>>::Item;

pub trait Viewable<'data> {
    type Fetch: Fetch<'data>;
}

pub trait SharedViewable<'data>: Viewable<'data> {}
