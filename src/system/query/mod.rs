use super::Fetch;

mod impls;

pub type QueryItem<'data, Q> = <<Q as Query<'data>>::Fetch as Fetch<'data>>::Item;

pub trait Query<'data> {
    type Fetch: Fetch<'data>;
}
