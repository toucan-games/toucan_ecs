use crate::component::marker::Not;
use crate::component::Component;
use crate::entity::Entity;
#[cfg(feature = "resource")]
use crate::resource::{marker, Resource};
use crate::system::foreach::fetch::*;
use crate::system::foreach::query::{Query, Sealed};

impl Sealed for () {}

impl<'data> Query<'data> for () {
    type Fetch = ();
}

impl Sealed for Entity {}

impl<'data> Query<'data> for Entity {
    type Fetch = FetchEntity;
}

impl<'data, C> Sealed for &'data C where C: Component {}

impl<'data, C> Query<'data> for &'data C
where
    C: Component,
{
    type Fetch = FetchRead<'data, C>;
}

impl<'data, C> Sealed for Option<&'data C> where C: Component {}

impl<'data, C> Query<'data> for Option<&'data C>
where
    C: Component,
{
    type Fetch = FetchOptionRead<'data, C>;
}

impl<C> Sealed for Not<C> where C: Component {}

impl<'data, C> Query<'data> for Not<C>
where
    C: Component,
{
    type Fetch = FetchNot<'data, C>;
}

impl<'data, C> Sealed for &'data mut C where C: Component {}

impl<'data, C> Query<'data> for &'data mut C
where
    C: Component,
{
    type Fetch = FetchWrite<'data, C>;
}

impl<'data, C> Sealed for Option<&'data mut C> where C: Component {}

impl<'data, C> Query<'data> for Option<&'data mut C>
where
    C: Component,
{
    type Fetch = FetchOptionWrite<'data, C>;
}

#[cfg(feature = "resource")]
impl<'data, R> Sealed for marker::Resource<'data, R> where R: Resource {}

#[cfg(feature = "resource")]
#[cfg_attr(docsrs, doc(cfg(feature = "resource")))]
impl<'data, R> Query<'data> for marker::Resource<'data, R>
where
    R: Resource,
{
    type Fetch = FetchResourceRead<'data, R>;
}

#[cfg(feature = "resource")]
impl<'data, R> Sealed for Option<marker::Resource<'data, R>> where R: Resource {}

#[cfg(feature = "resource")]
#[cfg_attr(docsrs, doc(cfg(feature = "resource")))]
impl<'data, R> Query<'data> for Option<marker::Resource<'data, R>>
where
    R: Resource,
{
    type Fetch = FetchResourceOptionRead<'data, R>;
}

#[cfg(feature = "resource")]
impl<'data, R> Sealed for marker::ResourceMut<'data, R> where R: Resource {}

#[cfg(feature = "resource")]
#[cfg_attr(docsrs, doc(cfg(feature = "resource")))]
impl<'data, R> Query<'data> for marker::ResourceMut<'data, R>
where
    R: Resource,
{
    type Fetch = FetchResourceWrite<'data, R>;
}

#[cfg(feature = "resource")]
impl<'data, R> Sealed for Option<marker::ResourceMut<'data, R>> where R: Resource {}

#[cfg(feature = "resource")]
#[cfg_attr(docsrs, doc(cfg(feature = "resource")))]
impl<'data, R> Query<'data> for Option<marker::ResourceMut<'data, R>>
where
    R: Resource,
{
    type Fetch = FetchResourceOptionWrite<'data, R>;
}
