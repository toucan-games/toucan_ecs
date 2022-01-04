pub mod pool;
pub mod set;
pub mod type_id;

pub trait Component: Copy + Send + Sync + 'static {}

impl<T> Component for T where T: Copy + Send + Sync + 'static {}
