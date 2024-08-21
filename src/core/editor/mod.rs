use std::{io::{stdout, Stdout}, ops::Deref};

use anyhow::Ok;
use crossterm::{cursor::MoveTo, event::{self, read}, style::{Color, Print, PrintStyledContent, Stylize}, terminal, ExecutableCommand, QueueableCommand};
use viewport::Viewport;
use std::io::Write;

use status_bar::StatusBar;
use command_bar::CommandBar;

use super::{buffer::Buffer, time::Timer};

// mods
mod modules; 
mod status_bar;
mod command_bar;
mod viewport;

enum Action{ // Possible movement actions

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
            event::KeyCode::Char(':') => Ok(Some(Action::SetMode(Mode::Command))),

            event::KeyCode::Up | event::KeyCode::Char('k') => Ok(Some(Action::MoveUp)),
            event::KeyCode::Down | event::KeyCode::Char('j') => Ok(Some(Action::MoveDown)),
            event::KeyCode::Left | event::KeyCode::Char('h') => Ok(Some(Action::MoveLeft)),
            event::KeyCode::Right | event::KeyCode::Char('l') => Ok(Some(Action::MoveRight)),

            event::KeyCode::Enter => Ok(Some(Action::EnterKey)),

            event::KeyCode::Char('i') => Ok(Some(Action::SetMode(Mode::Insert))),
            _ => Ok(None),
        },
        _ => Ok(None),
    }
}

fn handle_insert_event(ev: event::Event) -> anyhow::Result<Option<Action>> {
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
            event::KeyCode::Backspace => Ok(Some(Action::Backspace)),

            _ => Ok(None),
        },
        _ => Ok(None),
    }
}
pub struct Editor {
    pub cursor_x: u16,
    pub cursor_y: u16,

    running: bool,

    size: (u16, u16),

    current_file: Option<String>,
    mode: Mode,

    enable_status_bar: bool,
    status_bar: StatusBar,
    command_bar: CommandBar,

    buffer: Box<Buffer>,
    viewport: Viewport
}

impl Editor {
    pub fn new(buf: Buffer) -> Self {

        let _size = terminal::size().unwrap();

        Self {
            cursor_x: 0,
            cursor_y: 0,

            running: true,

            size: _size,

            current_file: None,
            mode: Mode::Normal,

            enable_status_bar: true,
            status_bar: StatusBar::new(),
            command_bar: CommandBar::new(),

            buffer: Box::new(buf),
            viewport: Viewport::new(_size)
        }
    }

    pub fn start(&mut self) -> anyhow::Result<()> {

        let mut _stdout = stdout();

        let mut timer = Timer::new();
        timer.start();

        terminal::enable_raw_mode()?;
        _stdout.execute(terminal::EnterAlternateScreen)?; // Enter to the upper terminal layer 
        _stdout.execute(terminal::Clear(terminal::ClearType::All))?; // Clear new terminal layer



        self.mainloop(&mut _stdout)?;


        _stdout.execute(terminal::LeaveAlternateScreen)?; // Leave upper terminal layer
        terminal::disable_raw_mode()?;

        timer.end();

        let duration_sec = timer.get_duration_sec();
        println!("~{} took", duration_sec);

        Ok(())
    }

    // main loop of logic
    fn mainloop(&mut self, _stdout: &mut Stdout) -> anyhow::Result<()> {
        while self.running {
            self.status_bar.get_editor_info((self.mode.clone(), &self.buffer.file.clone().unwrap_or("No such file or directory".to_string())));
            // drawings
            self.draw(_stdout, self.size)?;
            _stdout.flush()?;

            _stdout.queue(MoveTo(self.cursor_x.into(), self.cursor_y.into()))?; // start cursor
            _stdout.flush()?; // output sync with Stdout

            if let Some(action) = handel_event(&self.mode, read()?)? {
                match action {
                    Action::SetMode(new_mode) => {
                        if new_mode == Mode::Command{
                            self.cursor_x = 1; 
                            self.cursor_y = self.size.1;
                        }
                        self.mode = new_mode;
                    },

                    Action::MoveUp => self.cursor_y = self.cursor_y.saturating_sub(1),
                    Action::MoveDown => self.cursor_y = self.cursor_y.saturating_add(1),
                    Action::MoveRight => self.cursor_x = self.cursor_x.saturating_add(1),
                    Action::MoveLeft => self.cursor_x = self.cursor_x.saturating_sub(1),


                    Action::Typing(v) => {
                        _stdout.queue(Print(v))?;
                        self.cursor_x = self.cursor_x.saturating_add(1);
                        
                        // add char to command in command mode
                        if self.mode == Mode::Command {
                            self.command_bar.command.push(v);
                        }
                    },
                    Action::EnterKey => {
                        self.handle_enter();
                    },
                    Action::TabKey => {
                        self.cursor_x = self.cursor_x.saturating_add(4);
                    },
                    Action::Backspace => {
                        self.backspace_limiter(_stdout)?; // bounding 
                    },
                }
            }
        }
        
        Ok(())
    }
    fn backspace_limiter(&mut self, _stdout: &mut Stdout) -> anyhow::Result<()> {
        match self.mode {
            Mode::Command => {
                if self.cursor_x > 1 {
                    _stdout.queue(MoveTo(self.cursor_x.saturating_sub(1), self.cursor_y))?;
                    _stdout.queue(PrintStyledContent(
                        " ".on(Color::Rgb {
                                r: 255,
                                g: 255,
                                b: 255
                            })
                        )
                    )?;
                    self.cursor_x = self.cursor_x.saturating_sub(1);
                    _stdout.flush()?;
                    if self.mode == Mode::Command {
                        self.command_bar.command.pop();
                    }
                }
            },
            Mode::Insert => {
                _stdout.queue(MoveTo(self.cursor_x.saturating_sub(1), self.cursor_y))?;
                _stdout.queue(Print(" "))?;
                self.cursor_x = self.cursor_x.saturating_sub(1);
                _stdout.flush()?;
            },
            _ => {}
        }

        Ok(())
    }

    fn handle_enter(&mut self) {
        match self.mode {
            Mode::Command => {
                self.execute_command(self.command_bar.command.clone());
                self.command_bar.command = String::new();
                self.command_bar.command.push(':');
            },
            Mode::Insert => {
                self.cursor_y = self.cursor_y.saturating_add(1);
                self.cursor_x = 0;
            },
            Mode::Normal => {
                self.cursor_y = self.cursor_y.saturating_add(1);
            }
        }
    }

    fn execute_command(&mut self, command: String) {
        if command.trim().to_string() == ":q".to_string() {
            self.running = false;
        }
    }

    fn draw(&mut self, _stdout: &mut Stdout, size: (u16, u16)) -> anyhow::Result<()> {
        self.draw_buffer(_stdout)?;
        self.status_bar.draw(_stdout, size)?;
        match self.mode {
            Mode::Command => {self.command_bar.draw(_stdout, size)?; 
                return Ok(())},
            _ => {self.command_bar.clean(_stdout, size)?; return Ok(())},
        }
    }

    pub fn draw_buffer(&mut self, _stdout: &mut Stdout) -> anyhow::Result<()> {

        for (i, line) in self.buffer.lines.iter().enumerate() {
            _stdout.queue(MoveTo(0, i as u16))?;
            _stdout.queue(Print(line))?;
        }

        Ok(())
    }
}