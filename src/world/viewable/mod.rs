use crate::world::fetch::Fetch;

mod tuple;

pub type ViewableItem<'data, V> = <<V as Viewable<'data>>::Fetch as Fetch<'data>>::Item;

pub trait Viewable<'data> {
    type Fetch: Fetch<'data>;
}
