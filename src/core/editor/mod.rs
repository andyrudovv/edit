use std::io::Stdout;

use anyhow::Ok;
use crossterm::{cursor::MoveTo, event::{self, read}, style::Print, terminal, QueueableCommand};
use std::io::Write;

use status_bar::StatusBar;
use command_bar::CommandBar;

// mods
mod modules; 
mod status_bar;
mod command_bar;

enum Action{ // Possible movement actions
    Quit,

    MoveUp,
    MoveDown,
    MoveLeft,
    MoveRight,

    Typing(char),
    EnterKey,
    TabKey,
    Backspace,

    SetMode(Mode)
}

#[derive(Clone, Copy, PartialEq)]
enum Mode{ // interactions modes
    Normal,
    Insert,
    Command,
}

fn handel_event(mode:&Mode, ev: event::Event) -> anyhow::Result<Option<Action>>{
    match mode {
        Mode::Normal => handle_normal_event(ev),
        Mode::Insert => handle_insert_event(ev),
        Mode::Command => handle_command_event(ev),
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

            event::KeyCode::Enter => Ok(Some(Action::EnterKey)),

            event::KeyCode::Char('i') => Ok(Some(Action::SetMode(Mode::Insert))),
            event::KeyCode::Char('w') => {Ok(Some(Action::SetMode(Mode::Command)))},
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
            event::KeyCode::Esc => Ok(Some(Action::SetMode(Mode::Normal))),
            event::KeyCode::Enter => Ok(Some(Action::EnterKey)),
            event::KeyCode::Tab => Ok(Some(Action::TabKey)),
            event::KeyCode::Backspace => Ok(Some(Action::Backspace)),
            _ => Ok(None),
        },
        _ => Ok(None),
    }
}

fn handle_command_event(ev: event::Event) -> anyhow::Result<Option<Action>> {
    match ev {
        event::Event::Key(event) => match event.code {
            event::KeyCode::Char(v) => Ok(Some(Action::Typing(v))),
            event::KeyCode::Esc => Ok(Some(Action::SetMode(Mode::Normal))),
            event::KeyCode::Enter => Ok(Some(Action::EnterKey)),
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
    status_bar: StatusBar,
    command_bar: CommandBar
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
            status_bar: StatusBar::new(),
            command_bar: CommandBar::new()
        }
    }

    pub fn start(&mut self, _stdout: &mut Stdout) -> anyhow::Result<()> {
        loop {
            self.status_bar.get_editor_info(self.mode.clone());
            // drawings
            self.draw(_stdout, self.size)?;
            _stdout.flush()?;

            _stdout.queue(MoveTo(self.cursor_x.into(), self.cursor_y.into()))?; // start cursor
            _stdout.flush()?; // output sync with Stdout

            if let Some(action) = handel_event(&self.mode, read()?)? {
                match action {
                    Action::Quit => break,

                    Action::SetMode(new_mode) => self.mode = new_mode,

                    Action::MoveUp => self.cursor_y = self.cursor_y.saturating_sub(1),
                    Action::MoveDown => self.cursor_y = self.cursor_y.saturating_add(1),
                    Action::MoveRight => self.cursor_x = self.cursor_x.saturating_add(1),
                    Action::MoveLeft => self.cursor_x = self.cursor_x.saturating_sub(1),


                    Action::Typing(v) => {
                        _stdout.queue(Print(v))?;
                        self.cursor_x = self.cursor_x.saturating_add(1);
                    },
                    Action::EnterKey => {
                        self.cursor_y = self.cursor_y.saturating_add(1);
                    },
                    Action::TabKey => {
                        self.cursor_x = self.cursor_x.saturating_add(4);
                    },
                    Action::Backspace => {
                        _stdout.queue(Print(' '))?;
                        self.cursor_x = self.cursor_x.saturating_sub(1);
                    },
                    _ => {}
                }
            }
        }
        
        Ok(())
    }

    fn draw(&mut self, _stdout: &mut Stdout, size: (u16, u16)) -> anyhow::Result<()> {
        self.status_bar.draw(_stdout, size)?;
        match self.mode {
            Mode::Command => {self.command_bar.draw(_stdout, size, &mut self.cursor_x,&mut self.cursor_y)?; return Ok(())},
            _ => {self.command_bar.clean(_stdout, size)?; return Ok(())},
        }
    }
}