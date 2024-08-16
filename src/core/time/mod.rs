use chrono::prelude::*;

pub struct Timer {
    start_time: Option<DateTime<Local>>,
    end_time: Option<DateTime<Local>>
}

impl Timer {
    pub fn new() -> Self {
        Self { start_time: None, end_time: None }
    }
    pub fn start(&mut self) {
        self.start_time = Some(Local::now());
    }
    pub fn end(&mut self) {
        self.end_time = Some(Local::now());
    }

    pub fn get_duration_sec(&self) -> i64 {
        let s_time = self.start_time.unwrap();
        let e_time = self.end_time.unwrap();

        let duration = e_time - s_time;

        duration.num_seconds()
    }
}