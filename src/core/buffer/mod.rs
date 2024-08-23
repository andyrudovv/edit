
pub struct Buffer {
    pub file: Option<String>,
    pub lines: Vec<String>
}

impl Buffer {
    pub fn from_file(file: Option<String>) -> Self {
        let mut file_name = file;
        let lines = match &file_name {
            Some(file) => 
                {
                    let strings = std::fs::read_to_string(file);

                    match strings {
                        Ok(str) => str.lines().map(|s| s.to_string()).collect(),
                        Err(_) => {
                            file_name = Some("[New File]".to_string());
                            vec![String::new()]}
                    }
                },
            None => vec![]
        };

        Self {
            file:file_name, 
            lines
        }
    }

    pub fn get(&self, line: usize) -> Option<String> {
        if self.lines.len() >= line {
            return Some(self.lines[line].clone());
        }
        None
    }

    pub fn get_file_lenght(&self) -> usize {
        self.lines.len()
    }
}