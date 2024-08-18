use std::io::Stdout;

use anyhow::Ok;
use crossterm::{cursor::MoveTo, event::{self, read}, terminal, QueueableCommand};
use std::io::Write;

use status_bar::StatusBar;

// mods
mod modules; 
mod status_bar;

enum Action{ // Possible movement actions
    Quit,

    MoveUp,
    MoveDown,
    MoveLeft,
    MoveRight,

    Typing(char)
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
            event::KeyCode::Up | event::KeyCode::Char('k') => Ok(Some(Action::MoveUp)),
            event::KeyCode::Down | event::KeyCode::Char('j') => Ok(Some(Action::MoveDown)),
            event::KeyCode::Left | event::KeyCode::Char('h') => Ok(Some(Action::MoveLeft)),
            event::KeyCode::Right | event::KeyCode::Char('l') => Ok(Some(Action::MoveRight)),
            _ => Ok(None),
        },
        _ => Ok(None),
    }
}

fn handle_insert_event(ev: event::Event) -> anyhow::Result<Option<Action>> {
    //unimplemented!("Insert event: {ev:?}");
    match ev {
        event::Event::Key(event) => match event.code {
            event::KeyCode::Char(v) => Ok(Some(Action::Typing(v))),
            _ => Ok(None),
        },
        _ => Ok(None),
    }
}

pub struct Editor {
    pub cursor_x: u16,
    pub cursor_y: u16,

    size: (u16, u16),

    current_file: Option<String>,
    mode: Mode,

    enable_status_bar: bool,
    status_bar: StatusBar
}

impl Editor {
    pub fn new() -> Self {
        Self {
            cursor_x: 0,
            cursor_y: 0,

            size: terminal::size().unwrap(),

            current_file: None,
            mode: Mode::Normal,

            enable_status_bar: true,
            status_bar: StatusBar::new()
        }
    }

    pub fn start(&mut self, _stdout: &mut Stdout) -> anyhow::Result<()> {
        //self.init_modules(); // modules initialization
        loop {
            // drawings
            self.draw(_stdout, self.size)?;
            _stdout.flush()?;

            _stdout.queue(MoveTo(self.cursor_x.into(), self.cursor_y.into()))?; // start cursor
            _stdout.flush()?; // output sync with Stdout

            if let Some(action) = handel_event(&self.mode, read()?)? {
                match action {
                    Action::Quit => break,
                    Action::MoveUp => self.cursor_y = self.cursor_y.saturating_sub(1),
                    Action::MoveDown => self.cursor_y = self.cursor_y.saturating_add(1),
                    Action::MoveRight => self.cursor_x = self.cursor_x.saturating_add(1),
                    Action::MoveLeft => self.cursor_x = self.cursor_x.saturating_sub(1),
                    _ => {}
                }
            }
        }
        
        Ok(())
    }

    fn draw(&mut self, _stdout: &mut Stdout, size: (u16, u16)) -> anyhow::Result<()> {

        self.status_bar.draw(_stdout, size)?;

        Ok(())
    }
}