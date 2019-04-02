extern crate time;

pub struct Timer {
    start_time: time::Tm,
    end_time: time::Tm,
    pub elapsed: time::Tm
}

impl Timer {
    pub fn new() -> Timer {
        Timer {
            start_time: time::now(),
            end_time:   time::now(),
            elapsed:    time::now()
        }
    }
    pub fn start(&mut self) {
        self.start_time = time::now();
    }
    pub fn stop(&mut self) {
        self.end_time = time::now();
    }
    pub fn get_elapsed_in_mcs(&mut self) -> i64 {
        (self.end_time - self.start_time).num_microseconds().unwrap()
    }
}