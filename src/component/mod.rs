use std::any::Any;

pub mod pool;

pub trait Component: Copy + Any {}

impl<T> Component for T where T: Copy + Any {}
