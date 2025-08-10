use std::io::Stdout;

use crate::core::editor::Mode;

use super::BarModule;
use super::Info;

pub struct CurrentModeModule {
    enable: bool,
    current_mode: Mode
}

impl BarModule for CurrentModeModule {
    fn enable(&mut self) {
        self.enable = true;
    }
    fn disable(&mut self) {
        self.enable = false;
    }
    
    fn give_display(&mut self) -> String {
        self.get_mode_string()
    }
    
    fn get_editor_info(&mut self, info: Info) {
        self.current_mode = info.0;
    }
    
}

impl CurrentModeModule {
    pub fn new() -> Self{
        Self{
            enable: true,
            current_mode: Mode::Normal,
        }
    }

    pub fn get_mode_string(&mut self) -> String {
        match self.current_mode {
            Mode::Normal => "Normal".to_string(),
            Mode::Insert => "Insert".to_string(),
            Mode::Command => "Command".to_string(),
        }
    }
}

