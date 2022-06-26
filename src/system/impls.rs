use super::System;

impl<'data, F> System<'data, ()> for F
where
    F: FnMut() + 'data,
{
    fn run(&mut self, _: ()) {
        self()
    }
}
