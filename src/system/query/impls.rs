use crate::component::Component;
use crate::system::fetch::*;
use crate::system::foreach::{ForeachHolder, Query as ForeachQuery};
use crate::system::query::Query;
use crate::world::query;
use crate::world::view::{View, ViewMut, ViewOne, ViewOneMut};
#[cfg(feature = "resource")]
use crate::{marker::*, resource::Resource};

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

#[cfg(feature = "resource")]
#[cfg_attr(docsrs, doc(cfg(feature = "resource")))]
impl<'data, R> Query<'data> for Res<'data, R>
where
    R: Resource,
{
    type Fetch = FetchResourceRead<R>;
}

#[cfg(feature = "resource")]
#[cfg_attr(docsrs, doc(cfg(feature = "resource")))]
impl<'data, R> Query<'data> for ResMut<'data, R>
where
    R: Resource,
{
    type Fetch = FetchResourceWrite<R>;
}

#[cfg(feature = "resource")]
#[cfg_attr(docsrs, doc(cfg(feature = "resource")))]
impl<'data, R> Query<'data> for Option<Res<'data, R>>
where
    R: Resource,
{
    type Fetch = FetchResourceOptionRead<R>;
}

#[cfg(feature = "resource")]
#[cfg_attr(docsrs, doc(cfg(feature = "resource")))]
impl<'data, R> Query<'data> for Option<ResMut<'data, R>>
where
    R: Resource,
{
    type Fetch = FetchResourceOptionWrite<R>;
}
