use std::io::Stdout;
use chrono::prelude::*;

use super::BarModule;
use super::Rgb;

pub struct CurrentTimeModule {
    updatable: bool,

    current_time: DateTime<Local>,
    enable: bool,
}

impl BarModule for CurrentTimeModule {
    fn enable(&mut self) {
        self.enable = true;
    }
    fn disable(&mut self) {
        self.enable = false;
    }

    fn give_display(&mut self) -> String {
        self.get_time_string()
    }
}

impl CurrentTimeModule {
    pub fn new() -> Self {
        Self {
            current_time: Local::now(),
            updatable: true,
            enable: false,
        }
    }

    pub fn update(&mut self) {
        self.current_time = Local::now();
    }

    pub fn get_time_string(&mut self) -> String {
        
        self.update();

        let mut _hour = self.current_time.hour().to_string();
        let mut _minute = self.current_time.minute().to_string();

        if _hour.len() == 1 {
            _hour = "0".to_string() + &_hour;
        }
        if _minute.len() == 1 {
            _minute = "0".to_string() + &_minute;
        }
        
        let formatted_time = format!("{}:{}", _hour, _minute);

        formatted_time
    }
}
