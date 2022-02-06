use super::Fetch;

mod tuple;

pub type ViewableItem<'data, V> = <<V as Viewable<'data>>::Fetch as Fetch<'data>>::Item;

pub trait Viewable<'data> {
    #[doc(hidden)]
    type Fetch: Fetch<'data>;
}

pub trait SharedViewable<'data>: Viewable<'data> {}
