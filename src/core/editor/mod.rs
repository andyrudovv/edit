use std::io::Stdout;

use crossterm::{terminal, event::read, ExecutableCommand};

pub trait BarModule {
    fn enable(&self);
    fn disable(&self);
}

pub struct Editor {
    cursor_x: u8,
    cursor_y: u8,

    current_file: String,

    enable_status_bar: bool,
    modules: Vec<Box<dyn BarModule>>
}

impl Editor {
    pub fn new() -> Self {
        Self{
            cursor_x: 0,
            cursor_y: 0,

            current_file: "".to_string(),
            enable_status_bar: true,
            modules: Vec::new()
        }
    }

    pub fn init_modules(&self) {
        for module in self.modules.iter() {
            module.enable();
        }
    }

    pub fn draw(&self, _stdout: &Stdout) {
        loop {

        }
    }
}


// stdout.queue(cursor::MoveTo(cx, cy));
// match read()? {
//      crossterm::event::KeyCode::Char('q') => break,
//      crossterm::event::KeyCode::Char(v) => {print}
//}