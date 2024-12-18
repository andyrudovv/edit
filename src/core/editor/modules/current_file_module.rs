use std::io::Stdout;

use super::BarModule;

use super::Info;

pub struct CurrentFileModule {
    path: String,
    file: String,
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
    
    fn get_editor_info(&mut self, info: Info) {
        self.file = *info.1.clone();
    }
}

impl CurrentFileModule {
    pub fn new() -> Self {
        Self {
            path: " ".to_string(),
            file: " ".to_string(),
            enable: false
        }
    }
    pub fn get_path(&self) -> String {
        self.file.clone()
    }

    pub fn get_file_name(&self) -> &str {
        // TODO: implement this method returns current file name
        unimplemented!()
    }

    pub fn change_path(&mut self, new_path: &str) {
        self.path = new_path.to_string();
    }
}

