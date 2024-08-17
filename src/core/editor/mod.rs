use std::io::Stdout;

use crossterm::{cursor::MoveTo, event::{self, read}, QueueableCommand};
use std::io::Write;

use module::BarModule;

// mods
mod module; 

enum Action{ // Possible movement actions
    Quit,

    MoveUp,
    MoveDown,
    MoveLeft,
    MoveRight,
}

enum Mode{ // interactions modes
    Normal,
    Insert,
}

fn handel_event(mode:&Mode, ev: event::Event) -> anyhow::Result<Option<Action>>{
    match mode {
        Mode::Normal => handle_normal_event(ev),
        Mode::Insert => handle_insert_event(ev),
    }
}

fn handle_normal_event(ev: event::Event) -> anyhow::Result<Option<Action>>{
    match ev {
        event::Event::Key(event) => match event.code {
            event::KeyCode::Char('q') => Ok(Some(Action::Quit)),
            event::KeyCode::Up => Ok(Some(Action::MoveUp)),
            event::KeyCode::Down => Ok(Some(Action::MoveDown)),
            event::KeyCode::Left => Ok(Some(Action::MoveLeft)),
            event::KeyCode::Right => Ok(Some(Action::MoveRight)),
            _ => Ok(None),
        },
        _ => Ok(None),
    }
}

fn handle_insert_event(ev: event::Event) -> anyhow::Result<Option<Action>>{
    unimplemented!("Insert event: {ev:?}");
}



pub struct Editor {
    pub cursor_x: u8,//сделал публичными 
    pub cursor_y: u8,//сделал публичными 

    current_file: String,
    mode: Mode,

    enable_status_bar: bool,
    modules: Vec<Box<dyn BarModule>>
}

impl Editor {
    pub fn new() -> Self {
        Self{
            cursor_x: 0,
            cursor_y: 0,

            current_file: "".to_string(),
            mode: Mode::Normal,

            enable_status_bar: true,
            modules: Vec::new()
        }
    }

    pub fn init_modules(&self) {
        for module in self.modules.iter() {
            module.enable();
        }
    }

    pub fn start(&self, _stdout: &mut Stdout) -> anyhow::Result<()> {
        self.init_modules(); // modules initialization
        loop {
            _stdout.queue(MoveTo(self.cursor_x.into(), self.cursor_y.into()))?; // start cursor
            _stdout.flush()?; // output sync with Stdout

            if let Some(action) = handel_event(&self.mode, read()?)?{
                match action {
                    Action::Quit => break,
                    _=>{}
                }
            }
        }
        Ok(())
    }
}


// stdout.queue(cursor::MoveTo(cx, cy));
// match read()? {
//      crossterm::event::KeyCode::Char('q') => break,
//      crossterm::event::KeyCode::Char(v) => {print}
//}