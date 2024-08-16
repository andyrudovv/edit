use std::{alloc::handle_alloc_error, io::Stdout};

use crossterm::{event::{self, read, Event}, terminal, ExecutableCommand};
use std::io::{stdout, Write};
pub enum Action{//возможные события перемещения
    Quit,

    MoveUp,
    MoveDown,
    MoveLeft,
    MoveRight,
}

pub enum Mode{//режимы
    Normal,
    Insert,
}
//impl Mode{   он передумал делать так о оставил просто метод
pub fn handel_event(mode:&Mode, ev: event::Event) -> anyhow::Result<Option<Action>>{
    match mode {
        Mode::Normal => handle_normal_event(ev),
        Mode::Insert => handle_insert_event(ev),
    }
}
//}

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
pub trait BarModule {
    fn enable(&self);
    fn disable(&self);
}

pub struct Editor {
    pub cursor_x: u8,//сделал публичными 
    pub cursor_y: u8,//сделал публичными 

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