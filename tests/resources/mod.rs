#[derive(Default, Debug)]
pub struct SimpleResource {
    inner: i32,
}

impl SimpleResource {
    pub fn inner(&self) -> i32 {
        self.inner
    }

    #[allow(dead_code)]
    pub fn set_inner(&mut self, inner: i32) {
        self.inner = inner;
    }
}
