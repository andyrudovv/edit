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
}

impl CurrentFileModule {
    pub fn get_path(&self) -> String {
        self.path.clone()
    }

    pub fn get_file_name(&self) -> &str {
        // TODO: implement this method returns current file name
        unimplemented!()
    }
}

