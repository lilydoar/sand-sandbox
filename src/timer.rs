use std::time::{Duration, Instant};

pub struct Timer {
    duration: Duration,
    start_time: Instant,
}

impl Timer {
    pub fn new(duration: Duration) -> Self {
        Self {
            duration,
            start_time: Instant::now(),
        }
    }

    pub fn check_with_reset(&mut self) -> bool {
        let elapsed_time = Instant::now().duration_since(self.start_time);

        if elapsed_time >= self.duration {
            self.start_time = Instant::now();
            return true;
        }

        false
    }
}
