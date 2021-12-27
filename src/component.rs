use std::any::Any;

pub trait Component: Copy + Any {}

impl<T> Component for T where T: Copy + Any {}
