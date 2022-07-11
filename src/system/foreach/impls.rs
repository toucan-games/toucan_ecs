use super::ForeachSystem;

impl<'data, F> ForeachSystem<'data, ()> for F
where
    F: FnMut() + 'data,
{
    fn run(&mut self, _: ()) {
        self()
    }
}
