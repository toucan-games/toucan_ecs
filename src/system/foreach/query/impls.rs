use crate::component::Component;
use crate::entity::Entity;
use crate::marker::*;
#[cfg(feature = "resource")]
use crate::resource::Resource;
use crate::system::foreach::fetch::*;
use crate::system::foreach::query::Query;

impl<'data> Query<'data> for () {
    type Fetch = ();
}

impl<'data> Query<'data> for Entity {
    type Fetch = FetchEntity;
}

impl<'data, C> Query<'data> for &'data C
where
    C: Component,
{
    type Fetch = FetchRead<'data, C>;
}

impl<'data, C> Query<'data> for Option<&'data C>
where
    C: Component,
{
    type Fetch = FetchOptionRead<'data, C>;
}

impl<'data, C> Query<'data> for Not<C>
where
    C: Component,
{
    type Fetch = FetchNot<'data, C>;
}

impl<'data, C> Query<'data> for &'data mut C
where
    C: Component,
{
    type Fetch = FetchWrite<'data, C>;
}

impl<'data, C> Query<'data> for Option<&'data mut C>
where
    C: Component,
{
    type Fetch = FetchOptionWrite<'data, C>;
}

#[cfg(feature = "resource")]
#[cfg_attr(docsrs, doc(cfg(feature = "resource")))]
impl<'data, R> Query<'data> for Res<'data, R>
where
    R: Resource,
{
    type Fetch = FetchResourceRead<'data, R>;
}

#[cfg(feature = "resource")]
#[cfg_attr(docsrs, doc(cfg(feature = "resource")))]
impl<'data, R> Query<'data> for Option<Res<'data, R>>
where
    R: Resource,
{
    type Fetch = FetchResourceOptionRead<'data, R>;
}

#[cfg(feature = "resource")]
#[cfg_attr(docsrs, doc(cfg(feature = "resource")))]
impl<'data, R> Query<'data> for ResMut<'data, R>
where
    R: Resource,
{
    type Fetch = FetchResourceWrite<'data, R>;
}

#[cfg(feature = "resource")]
#[cfg_attr(docsrs, doc(cfg(feature = "resource")))]
impl<'data, R> Query<'data> for Option<ResMut<'data, R>>
where
    R: Resource,
{
    type Fetch = FetchResourceOptionWrite<'data, R>;
}
