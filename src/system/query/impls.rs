use crate::component::Component;
#[cfg(feature = "resource")]
use crate::resource::{marker, Resource};
#[cfg(feature = "resource")]
use crate::system::fetch::*;
use crate::system::foreach::{ForeachHolder, Query as ForeachQuery};
use crate::system::query::{Query, Sealed};
use crate::world::query;
use crate::world::view::{View, ViewMut, ViewOne, ViewOneMut};

impl Sealed for () {}

impl<'data> Query<'data> for () {
    type Fetch = ();
}

impl<'data, C> Sealed for ViewOne<'data, C> where C: Component {}

impl<'data, C> Query<'data> for ViewOne<'data, C>
where
    C: Component,
{
    type Fetch = FetchViewOne<C>;
}

impl<'data, C> Sealed for ViewOneMut<'data, C> where C: Component {}

impl<'data, C> Query<'data> for ViewOneMut<'data, C>
where
    C: Component,
{
    type Fetch = FetchViewOneMut<C>;
}

impl<'data, Q> Sealed for View<'data, Q> where Q: query::Query<'data> {}

impl<'data, Q> Query<'data> for View<'data, Q>
where
    Q: query::Query<'data>,
{
    type Fetch = FetchView<'data, Q>;
}

impl<'data, Q> Sealed for ViewMut<'data, Q> where Q: query::QueryMut<'data> {}

impl<'data, Q> Query<'data> for ViewMut<'data, Q>
where
    Q: query::QueryMut<'data>,
{
    type Fetch = FetchViewMut<'data, Q>;
}

impl<'data, Q> Sealed for ForeachHolder<'data, Q> where Q: ForeachQuery<'data> {}

impl<'data, Q> Query<'data> for ForeachHolder<'data, Q>
where
    Q: ForeachQuery<'data>,
{
    type Fetch = FetchForeachHolder<'data, Q>;
}

#[cfg(feature = "resource")]
impl<'data, R> Sealed for marker::Resource<'data, R> where R: Resource {}

#[cfg(feature = "resource")]
#[cfg_attr(docsrs, doc(cfg(feature = "resource")))]
impl<'data, R> Query<'data> for marker::Resource<'data, R>
where
    R: Resource,
{
    type Fetch = FetchResourceRead<R>;
}

#[cfg(feature = "resource")]
impl<'data, R> Sealed for marker::ResourceMut<'data, R> where R: Resource {}

#[cfg(feature = "resource")]
#[cfg_attr(docsrs, doc(cfg(feature = "resource")))]
impl<'data, R> Query<'data> for marker::ResourceMut<'data, R>
where
    R: Resource,
{
    type Fetch = FetchResourceWrite<R>;
}

#[cfg(feature = "resource")]
impl<'data, R> Sealed for Option<marker::Resource<'data, R>> where R: Resource {}

#[cfg(feature = "resource")]
#[cfg_attr(docsrs, doc(cfg(feature = "resource")))]
impl<'data, R> Query<'data> for Option<marker::Resource<'data, R>>
where
    R: Resource,
{
    type Fetch = FetchResourceOptionRead<R>;
}

#[cfg(feature = "resource")]
impl<'data, R> Sealed for Option<marker::ResourceMut<'data, R>> where R: Resource {}

#[cfg(feature = "resource")]
#[cfg_attr(docsrs, doc(cfg(feature = "resource")))]
impl<'data, R> Query<'data> for Option<marker::ResourceMut<'data, R>>
where
    R: Resource,
{
    type Fetch = FetchResourceOptionWrite<R>;
}
