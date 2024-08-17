use std::io::Stdout;

use current_file_module::CurrentFileModule;
use current_time_module::CurrentTimeModule;

pub mod current_file_module;
pub mod current_time_module;

pub trait BarModule {
    fn enable(&mut self);
    fn disable(&mut self);

    fn give_display(&mut self) -> String;
}


pub fn get_modules() -> Vec<Box<dyn BarModule>> {
    vec![
        Box::new(CurrentTimeModule::new()),
        Box::new(CurrentFileModule::new()),
    ]
}

