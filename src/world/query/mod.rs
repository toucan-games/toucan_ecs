use super::fetch::{Fetch, FetchMut};

mod tuple;

pub type QueryItem<'data, Q> = <<Q as Query<'data>>::Fetch as Fetch<'data>>::Item;

pub trait Query<'data> {
    type Fetch: Fetch<'data>;
}

pub type QueryMutItem<'data, Q> = <<Q as QueryMut<'data>>::Fetch as FetchMut<'data>>::Item;

pub trait QueryMut<'data> {
    type Fetch: FetchMut<'data>;
}
