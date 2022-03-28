use crate::world::fetch::Fetch;

mod tuple;

pub type QueryItem<'data, Q> = <<Q as Query<'data>>::Fetch as Fetch<'data>>::Item;

pub trait Query<'data> {
    type Fetch: Fetch<'data>;
}

pub trait QueryShared<'data>: Query<'data> {}
