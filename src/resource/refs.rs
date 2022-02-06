use std::marker::PhantomData;
use std::ops::{Deref, DerefMut};
use std::sync::MutexGuard;

use super::Resource;

/// Shared borrow of the resource.
#[allow(non_snake_case)]
#[repr(transparent)]
pub struct Ref<'data, R>
where
    R: Resource,
{
    __: MutexGuard<'data, Box<dyn Resource>>,
    ___: PhantomData<&'data R>,
}

impl<'data, R> Ref<'data, R>
where
    R: Resource,
{
    pub(super) fn new(data: MutexGuard<'data, Box<dyn Resource>>) -> Self {
        Self {
            __: data,
            ___: PhantomData,
        }
    }
}

impl<'data, R> Deref for Ref<'data, R>
where
    R: Resource,
{
    type Target = R;

    // noinspection DuplicatedCode
    fn deref(&self) -> &Self::Target {
        let data = self.__.deref();
        data.as_ref()
            .as_any_ref()
            .downcast_ref()
            .expect("downcast error")
    }
}

/// Unique borrow of the resource.
#[allow(non_snake_case)]
#[repr(transparent)]
pub struct RefMut<'data, R>
where
    R: Resource,
{
    __: MutexGuard<'data, Box<dyn Resource>>,
    ___: PhantomData<&'data R>,
}

impl<'data, R> RefMut<'data, R>
where
    R: Resource,
{
    pub(super) fn new(data: MutexGuard<'data, Box<dyn Resource>>) -> Self {
        Self {
            __: data,
            ___: PhantomData,
        }
    }
}

impl<'data, R> Deref for RefMut<'data, R>
where
    R: Resource,
{
    type Target = R;

    // noinspection DuplicatedCode
    fn deref(&self) -> &Self::Target {
        let data = self.__.deref();
        data.as_ref()
            .as_any_ref()
            .downcast_ref()
            .expect("downcast error")
    }
}

impl<'data, R> DerefMut for RefMut<'data, R>
where
    R: Resource,
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        let data = self.__.deref_mut();
        data.as_mut()
            .as_any_mut()
            .downcast_mut()
            .expect("downcast error")
    }
}
