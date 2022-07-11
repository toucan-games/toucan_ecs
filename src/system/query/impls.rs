use crate::component::Component;
use crate::fetch::{FetchForeachHolder, FetchView, FetchViewMut, FetchViewOne, FetchViewOneMut};
#[cfg(feature = "resource")]
use crate::resource::{marker, Resource};
use crate::system::fetch::*;
use crate::system::foreach::{ForeachHolder, Query as ForeachQuery};
use crate::world::query;
use crate::world::view::{View, ViewMut, ViewOne, ViewOneMut};

use super::Query;

impl<'data> Query<'data> for () {
    type Fetch = ();
}

impl<'data, C> Query<'data> for ViewOne<'data, C>
where
    C: Component,
{
    type Fetch = FetchViewOne<C>;
}

impl<'data, C> Query<'data> for ViewOneMut<'data, C>
where
    C: Component,
{
    type Fetch = FetchViewOneMut<C>;
}

impl<'data, Q> Query<'data> for View<'data, Q>
where
    Q: query::Query<'data>,
{
    type Fetch = FetchView<'data, Q>;
}

impl<'data, Q> Query<'data> for ViewMut<'data, Q>
where
    Q: query::QueryMut<'data>,
{
    type Fetch = FetchViewMut<'data, Q>;
}

impl<'data, Q> Query<'data> for ForeachHolder<'data, Q>
where
    Q: ForeachQuery<'data>,
{
    type Fetch = FetchForeachHolder<'data, Q>;
}

cfg_resource! {
    impl<'data, R> Query<'data> for marker::Resource<'data, R>
    where
        R: Resource,
    {
        type Fetch = FetchResourceRead<R>;
    }

    impl<'data, R> Query<'data> for marker::ResourceMut<'data, R>
    where
        R: Resource,
    {
        type Fetch = FetchResourceWrite<R>;
    }

    impl<'data, R> Query<'data> for Option<marker::Resource<'data, R>>
    where
        R: Resource,
    {
        type Fetch = FetchResourceOptionRead<R>;
    }

    impl<'data, R> Query<'data> for Option<marker::ResourceMut<'data, R>>
    where
        R: Resource,
    {
        type Fetch = FetchResourceOptionWrite<R>;
    }
}
