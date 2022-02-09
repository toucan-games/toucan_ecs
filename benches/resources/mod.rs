use std::time::Instant;

pub struct Time {
    start: Instant,
}

impl Time {
    pub fn new() -> Self {
        Self {
            start: Instant::now(),
        }
    }

    pub fn elapsed_secs(&self) -> f32 {
        self.start.elapsed().as_secs_f32()
    }

    #[allow(dead_code)]
    pub fn reset(&mut self) {
        self.start = Instant::now();
    }
}
