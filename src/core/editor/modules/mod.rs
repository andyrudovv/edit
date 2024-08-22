use current_file_module::CurrentFileModule;
use current_time_module::CurrentTimeModule;
use current_mode_module::CurrentModeModule;

use super::Mode;

pub mod current_file_module;
pub mod current_time_module;
pub mod current_mode_module;

pub type Rgb = (u8, u8, u8);

pub type Info = (Mode, Box<String>);

pub trait BarModule {
    fn enable(&mut self);
    fn disable(&mut self);

    fn give_display(&mut self) -> String;
                                      //(Mode, File name)
    fn get_editor_info(&mut self, info: Info); 
}


pub fn get_modules() -> Vec<Box<dyn BarModule>> {
    vec![
        Box::new(CurrentModeModule::new()),
        Box::new(CurrentTimeModule::new()),
        Box::new(CurrentFileModule::new()),
    ]
}

