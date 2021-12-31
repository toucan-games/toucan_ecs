pub mod pool;

pub trait Component: 'static + Copy + Send + Sync {}

impl<T> Component for T where T: 'static + Copy + Send + Sync {}
