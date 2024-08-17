use std::io::Stdout;

use super::BarModule;

pub struct CurrentFileModule {
    path: String,
    enable: bool
}

impl BarModule for CurrentFileModule {
    fn enable(&mut self) {
        self.enable = true;
    }
    fn disable(&mut self) {
        self.enable = false;
    }
    
    fn give_display(&mut self) -> String {
        self.get_path()
    }
}

impl CurrentFileModule {
    pub fn new() -> Self {
        Self {
            path: "src/main.rs".to_string(),
            enable: false
        }
    }
    pub fn get_path(&self) -> String {
        self.path.clone()
    }

    pub fn get_file_name(&self) -> &str {
        // TODO: implement this method returns current file name
        unimplemented!()
    }

    pub fn change_path(&mut self, new_path: &str) {
        self.path = new_path.to_string();
    }
}

