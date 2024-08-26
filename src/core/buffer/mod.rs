
pub struct Buffer {
    pub file: Option<String>,
    pub lines: Vec<String>
}

impl Buffer {
    pub fn from_file(file: Option<String>) -> Self {
        let file_name = file;
        let lines = match &file_name {
            Some(file) => 
                {
                    let strings = std::fs::read_to_string(file);

                    match strings {
                        Ok(str) => str.lines().map(|s| s.to_string()).collect(),
                        Err(_) => {
                            vec![String::new()]
                        }
                    }
                },
            None => vec![String::new()]
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

    pub fn save(&self) -> anyhow::Result<()> {
        let mut whole_content = String::new();
        for line in self.lines.iter() {
            whole_content.push_str(line);
            whole_content.push('\n');
        }

        std::fs::write(self.file.clone().unwrap(), whole_content)?;

        Ok(())
    }

    pub fn save_by_name(&self) {

    }
}
