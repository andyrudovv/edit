use std::{any, borrow::Borrow, io::{stdout, Stdout}};

use anyhow::Ok;
use crossterm::{cursor::MoveTo, event::{self, read}, style::{Color, Print, PrintStyledContent, Stylize}, terminal, ExecutableCommand, QueueableCommand};
use std::io::Write;

use status_bar::StatusBar;
use command_bar::CommandBar;

use super::{buffer::Buffer, time::Timer};

// mods
mod modules; 
mod status_bar;
mod command_bar;

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

pub struct Editor {
    buffer: Buffer,
    viewport_top: u16,
    viewport_left: u16,

    pub cursor_x: u16,
    pub cursor_y: u16,

    stdout: Stdout,
    timer: Timer,
    running: bool,

    size: (u16, u16),

    mode: Mode,

    enable_status_bar: bool,
    status_bar: StatusBar,

    command_bar: CommandBar,

}


impl Editor {
    pub fn new(buf: Buffer) -> anyhow::Result<Self> {

        let mut _stdout = stdout();

        let mut timer = Timer::new();
        timer.start();

        terminal::enable_raw_mode()?;
        _stdout
            .execute(terminal::EnterAlternateScreen)? // Enter to the upper terminal layer 
            .execute(terminal::Clear(terminal::ClearType::All))?; // Clear new terminal layer

        let _size = terminal::size().unwrap();


        Ok(Editor {
                buffer: buf,
                viewport_left: 0,
                viewport_top: 0,

                cursor_x: 0,
                cursor_y: 0,

                running: true,

                size: _size,

                mode: Mode::Normal,

                enable_status_bar: true,
                status_bar: StatusBar::new(),
                command_bar: CommandBar::new(),

                timer,

                stdout: _stdout,
            })
    }

    pub fn start(&mut self) -> anyhow::Result<()> {

        self.mainloop()?;

        Ok(())
    }

    // main loop of logic
    fn mainloop(&mut self) -> anyhow::Result<()> {
        while self.running {
            self.status_bar.get_editor_info((
                self.mode.clone(), 
                Box::new(
                    self.buffer.file
                    .clone()
                    .unwrap_or("[No Name]".to_string()
                ))
            ));
            // drawings
            self.draw(self.size)?;
            self.stdout.flush()?;

            self.stdout.queue(MoveTo(self.cursor_x.into(), self.cursor_y.into()))?; // start cursor
            self.stdout.flush()?; // output sync with Stdout

            if let Some(action) = self.handel_event(read()?)? {
                match action {
                    Action::SetMode(new_mode) => {
                        if new_mode == Mode::Command{
                            self.cursor_x = 1; 
                            self.cursor_y = self.size.1;
                        }
                        self.mode = new_mode;
                    },

                    Action::MoveUp => {
                        self.cursor_y = self.cursor_y.saturating_sub(1);
                    },
                    Action::MoveDown => {
                        self.cursor_y = self.cursor_y.saturating_add(1);
                        if self.cursor_y > self.viewport_height() as u16 {
                            self.cursor_y = self.viewport_height() as u16;
                        }
                    },
                    Action::MoveRight => {
                        self.cursor_x = self.cursor_x.saturating_add(1);
                    },
                    Action::MoveLeft => {
                        self.cursor_x = self.cursor_x.saturating_sub(1);
                        if self.cursor_x < self.viewport_left {
                            self.cursor_x = self.viewport_left;
                        }
                    },

                    Action::Typing(v) => {
                        self.handle_changing(v)?;
                    },
                    Action::EnterKey => {
                        self.handle_enter();
                    },
                    Action::TabKey => {
                        self.cursor_x = self.cursor_x.saturating_add(4);
                    },
                    Action::Backspace => {
                        self.backspace_limiter()?; // bounding 
                    },
                }
            }
        }
        
        Ok(())
    }

    fn handle_changing(&mut self, v: char) -> anyhow::Result<()> {
        let mut new_line = String::new();
        let old_line = self.buffer.lines[self.cursor_y as usize].clone();

        if self.cursor_x > old_line.len() as u16 {
            self.stdout.queue(MoveTo(0, self.cursor_y))?;
            self.cursor_x = old_line.len() as u16;
        }

        if self.cursor_x > 0 {
            let unchanged_left_part = &old_line[0..self.cursor_x as usize];
            let unchanged_right_part = &old_line[self.cursor_x as usize..old_line.len()];
            new_line.push_str(unchanged_left_part);
            new_line.push(v);
            new_line.push_str(unchanged_right_part);

        }
        else if self.cursor_x as usize == old_line.len() + 1 {
            new_line.push_str(&old_line);
            new_line.push(v);
        }
        else {
            new_line.push(v);
            new_line.push_str(&old_line);
        }

        //self.stdout.queue(Print(v))?;
        self.buffer.lines[self.cursor_y as usize] = new_line;
        self.cursor_x = self.cursor_x.saturating_add(1);
        
        // add char to command in command mode
        if self.mode == Mode::Command {
            self.command_bar.command.push(v);
        }

        Ok(())
    }

    fn backspace_limiter(&mut self) -> anyhow::Result<()> {
        match self.mode {
            Mode::Command => {
                if self.cursor_x > 1 {
                    self.stdout.queue(MoveTo(self.cursor_x.saturating_sub(1), self.cursor_y))?;
                    self.stdout.queue(PrintStyledContent(
                        " ".on(Color::Rgb {
                                r: 255,
                                g: 255,
                                b: 255
                            })
                        )
                    )?;
                    self.cursor_x = self.cursor_x.saturating_sub(1);
                    self.stdout.flush()?;
                    if self.mode == Mode::Command {
                        self.command_bar.command.pop();
                    }
                }
            },
            Mode::Insert => {
                let mut new_line = String::new();
                let old_line = self.buffer.lines[self.cursor_y as usize].clone();

                if self.cursor_x > 0 {
                    let unchanged_left_part = &old_line[0..self.cursor_x as usize - 1];
                    let unchanged_right_part = &old_line[self.cursor_x as usize..old_line.len()];
                    new_line.push_str(unchanged_left_part);
                    new_line.push_str(unchanged_right_part);

                }
                else if self.cursor_x as usize == old_line.len() + 1 {
                    new_line.push_str(&old_line[0..self.cursor_x as usize]);
                }
                else {
                    new_line.push_str(&old_line);
                }

                //self.stdout.queue(Print(v))?;
                self.buffer.lines[self.cursor_y as usize] = new_line;
                self.cursor_x = self.cursor_x.saturating_sub(1);
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

    fn draw(&mut self, size: (u16, u16)) -> anyhow::Result<()> {
        
        self.draw_status_bar()?;
        self.draw_viewport()?;

        match self.mode {
            Mode::Command => {
                self.draw_command_bar()?; 
                return Ok(())
            },
            _ => {self.command_bar.clean(&mut self.stdout, size)?; return Ok(())},
        }
    }

    pub fn viewport_line(&self, n: u16) -> Option<String> {
        let buffer_line = self.viewport_top + n;
        
        self.buffer.get(buffer_line as usize)
    }

    pub fn draw_viewport(&mut self) -> anyhow::Result<()> {
        for i in 0..self.viewport_height() {
            let line = match self.viewport_line(i as u16){
                Some(s) => s,
                None => String::new()
            };

            let w = self.viewport_width();
            self.stdout
                .queue(MoveTo(0, i as u16))?
                .queue(Print(format!(
                        "{line:<width$}",
                        width = w as usize
                    ))
                )?;
        }

        Ok(())
    }
    
    pub fn draw_status_bar(&mut self) -> anyhow::Result<()> {
        self.status_bar.draw(&mut self.stdout, self.size)?;

        Ok(())
    }
    
    pub fn draw_command_bar(&mut self) -> anyhow::Result<()> {
        self.command_bar.draw(&mut self.stdout, self.size)?;

        Ok(())
    }

    fn viewport_height(&self) -> usize {
        self.size.1 as usize - 2
    }

    fn viewport_width(&self) -> u16 {
        self.size.0
    }

    
    fn handel_event(&self, ev: event::Event) -> anyhow::Result<Option<Action>>{
        match self.mode {
            Mode::Normal => self.handle_normal_event(ev),
            Mode::Insert => self.handle_insert_event(ev),
            Mode::Command => self.handle_command_event(ev),
        }
    }

    fn handle_normal_event(&self, ev: event::Event) -> anyhow::Result<Option<Action>>{
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

    fn handle_insert_event(&self, ev: event::Event) -> anyhow::Result<Option<Action>> {
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

    fn handle_command_event(&self, ev: event::Event) -> anyhow::Result<Option<Action>> {
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
}


impl Drop for Editor {
    fn drop(&mut self) {
        self.stdout.execute(terminal::LeaveAlternateScreen); // Leave upper terminal layer
        terminal::disable_raw_mode();

        self.timer.end();

        let duration_sec = self.timer.get_duration_sec();
        println!("~{} took", duration_sec);

    }
}