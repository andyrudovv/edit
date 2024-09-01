pub struct EditorSettings {
    pub font_color: (u8, u8, u8),
}

pub struct StatusBarSettings {
    pub background_color: (u8, u8, u8),
    pub separator_symbol: String,
}

pub struct CommandBarSettings {
    pub background_color: (u8, u8, u8),
    pub font_color: (u8, u8, u8),
}

pub struct CommandsBindings {
    pub quit: String,
    pub save: String,
    pub edit: String,
}

impl EditorSettings {
    pub fn init() -> Self{
        let strings = 
        match std::fs::read_to_string("edit.conf") {
            Ok(v) => {v},
            Err(_) => return Self{
                font_color: (255,255,255),
            }
        };
        let lines:Vec<String> = strings.lines().map(|x| x.to_string()).collect();
        let mut custom_font_color:(u8,u8,u8) = (255,204,229);
        for i in lines {
            if i.starts_with("editor_font_color"){
                let temp:Vec<String>= i.split('=').map(|x| x.trim().to_string()).collect();
                let temp = temp.last().unwrap();
                let temp = &temp[1..temp.len()-1];
                let temp:Vec<u8> = temp.split(',').map(|x| x.trim().parse::<u8>().unwrap()).collect();
                custom_font_color.0 = temp[0];
                custom_font_color.1 = temp[1];
                custom_font_color.2 = temp[2];
            }
        }
        
        Self{
            font_color: custom_font_color,
        }
    }
    pub fn get_info_color(&self) -> Option<(u8,u8,u8)>{
        Some(self.font_color)
    }
    
}

impl StatusBarSettings {
    pub fn init() -> Self{
        
        let strings = 
        match std::fs::read_to_string("edit.conf") {
            Ok(v) => {v},
            Err(_) => return Self{
                background_color: (255,204,229),
                separator_symbol: String::from("◢◤"),
            }
        };
        let lines:Vec<String> = strings.lines().map(|x| x.to_string()).collect();
        let mut custom_color:(u8,u8,u8) = (255,204,229);
        let mut custom_separator = String::from("◢◤");
        for i in lines {
            if i.starts_with("status_bar_background_color"){
                let temp:Vec<String>= i.split('=').map(|x| x.trim().to_string()).collect();
                let temp = temp.last().unwrap();
                let temp = &temp[1..temp.len()-1];
                let temp:Vec<u8> = temp.split(',').map(|x| x.trim().parse::<u8>().unwrap()).collect();
                custom_color.0 = temp[0];
                custom_color.1 = temp[1];
                custom_color.2 = temp[2];
            }
            if i.starts_with("status_bar_separator"){
                let temp:Vec<String>= i.split('=').map(|x| x.trim().to_string()).collect();
                let temp = temp.last().unwrap();
                custom_separator = (&temp[1..temp.len()-1]).to_string();
            }
        }
        
        Self{
            background_color: custom_color,
            separator_symbol: custom_separator,
        }
    }
    pub fn get_info_backcolor(&self) -> Option<(u8,u8,u8)>{
        Some(self.background_color)
    }
    pub fn get_info_separator(&self) -> Option<String>{
        Some(self.separator_symbol.clone())
    }
}

impl CommandBarSettings {
    pub fn init() -> Self{
        let strings = 
        match std::fs::read_to_string("edit.conf") {
            Ok(v) => {v},
            Err(_) => return Self{
                background_color: (255,255,255),
                font_color: (0,0,0),
            }
        };
        let lines:Vec<String> = strings.lines().map(|x| x.to_string()).collect();
        let mut custom_backgroundcolor:(u8,u8,u8) = (255,204,229);
        let mut custom_font_color:(u8,u8,u8) = (0,0,0);
        for i in lines {
            if i.starts_with("command_bar_background_color"){
                let temp:Vec<String>= i.split('=').map(|x| x.trim().to_string()).collect();
                let temp = temp.last().unwrap();
                let temp = &temp[1..temp.len()-1];
                let temp:Vec<u8> = temp.split(',').map(|x| x.trim().parse::<u8>().unwrap()).collect();
                custom_backgroundcolor.0 = temp[0];
                custom_backgroundcolor.1 = temp[1];
                custom_backgroundcolor.2 = temp[2];
            }
            if i.starts_with("command_bar_font_color"){
                let temp:Vec<String>= i.split('=').map(|x| x.trim().to_string()).collect();
                let temp = temp.last().unwrap();
                let temp = &temp[1..temp.len()-1];
                let temp:Vec<u8> = temp.split(',').map(|x| x.trim().parse::<u8>().unwrap()).collect();
                custom_font_color.0 = temp[0];
                custom_font_color.1 = temp[1];
                custom_font_color.2 = temp[2];
            }
        }
        
        Self{
            background_color: custom_backgroundcolor,
            font_color: custom_font_color,
        }
    }
    pub fn get_info_backcolor(&self) -> Option<(u8,u8,u8)>{
        Some(self.background_color)
    }
    pub fn get_info_color(&self) -> Option<(u8,u8,u8)>{
        Some(self.font_color)
    }
}

impl CommandsBindings {
    pub fn init() -> Self{
        
        let strings = 
        match std::fs::read_to_string("edit.conf") {
            Ok(v) => {v},
            Err(_) => return Self{
                quit: String::from("q"),
                save: String::from("w"),
                edit: String::from("e")
            }
        };
        let lines:Vec<String> = strings.lines().map(|x| x.to_string()).collect();
        let mut custom_quit = String::from("q");
        let mut custom_save = String::from("w");
        let mut custom_edit = String::from("e");
        for i in lines {
            if i.starts_with("cmd_quit"){
                let temp:Vec<String>= i.split('=').map(|x| x.trim().to_string()).collect();
                let temp = temp.last().unwrap();
                custom_quit = (&temp[1..temp.len()-1]).to_string();
            }
            if i.starts_with("cmd_save"){
                let temp:Vec<String>= i.split('=').map(|x| x.trim().to_string()).collect();
                let temp = temp.last().unwrap();
                custom_save = (&temp[1..temp.len()-1]).to_string();
            }
            if i.starts_with("cmd_edit"){
                let temp:Vec<String>= i.split('=').map(|x| x.trim().to_string()).collect();
                let temp = temp.last().unwrap();
                custom_edit = (&temp[1..temp.len()-1]).to_string();
            }
        }
        
        Self{
            quit: custom_quit,
            save: custom_save,
            edit: custom_edit
        }
    }

    pub fn get_info_quit(&self) -> Option<String>{
        Some(self.quit.clone())
    }
    pub fn get_info_save(&self) -> Option<String>{
        Some(self.save.clone())
    }
    pub fn get_info_edit(&self) -> Option<String>{
        Some(self.edit.clone())
    }
}
