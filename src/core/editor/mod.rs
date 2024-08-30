use std::io::{stdout, Stdout};

use anyhow::Ok;
use crossterm::{
    cursor::{self, MoveTo},
    event::{self, read},
    style::{Color, Print, PrintStyledContent, Stylize},
    terminal, ExecutableCommand, QueueableCommand,
};
use std::io::Write;

use command_bar::CommandBar;
use status_bar::StatusBar;

use super::{buffer::Buffer, timer::Timer};

// mods
mod command_bar;
mod modules;
mod status_bar;

enum Action {
    // Possible movement actions
    MoveUp,
    MoveDown,
    MoveLeft,
    MoveRight,

    Typing(char),

    EnterKey,
    TabKey,
    Backspace,

    SetMode(Mode),
}

#[derive(Clone, Copy, PartialEq)]
enum Mode {
    // interactions modes
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
    scrolling_padding: u16,
}

impl Editor {
    pub fn new(buf: Buffer) -> anyhow::Result<Self> {
        let mut _stdout = stdout();

        let mut timer = Timer::new();
        timer.start();

        terminal::enable_raw_mode()?;
        _stdout
            .execute(terminal::EnterAlternateScreen)? // Enter to the upper terminal layer
            .execute(terminal::Clear(terminal::ClearType::All))? // Clear new terminal layer
            .execute(cursor::SetCursorStyle::BlinkingBar)?
            .execute(cursor::DisableBlinking)?;

        let _size = terminal::size().expect("Could not get size of terminal");

        Ok(Editor {
            buffer: buf,
            viewport_left: 0,
            viewport_top: 0,
            scrolling_padding: 1,

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
                Box::new(self.buffer.file.clone().unwrap_or("[No Name]".to_string())),
            ));
            // drawings
            self.draw(self.size)?;
            self.stdout.flush()?;

            self.stdout
                .queue(MoveTo(self.cursor_x as u16 + self.buffer.get_file_lenght().to_string().len() as u16+1, self.cursor_y.into()))?; // start cursor
            self.stdout.flush()?; // output sync with Stdout

            if let Some(action) = self.handel_event(read()?)? {
                match action {
                    Action::SetMode(new_mode) => {
                        if new_mode == Mode::Command {
                            self.cursor_x = 0;
                            self.cursor_y = self.size.1;
                        }
                        self.mode = new_mode;
                    }

                    Action::MoveUp => {
                        if self.cursor_y < self.scrolling_padding {
                            self.viewport_top = self.viewport_top.saturating_sub(1);
                        } else {
                            self.cursor_y = self.cursor_y.saturating_sub(1);
                        }

                        if self.cursor_y < self.scrolling_padding && self.viewport_top == 0 {
                            self.cursor_y = self.cursor_y.saturating_sub(1);
                        }
                    }
                    Action::MoveDown => {
                        let cannot_move_down = self.viewport_height()
                            >= (self.buffer.get_file_lenght() - self.viewport_top as usize);

                        if self.cursor_y + self.viewport_top
                            < self.buffer.get_file_lenght() as u16 - 1
                        {
                            if self.cursor_y
                                < self.viewport_height() as u16 - self.scrolling_padding
                                || cannot_move_down
                            {
                                self.cursor_y = self.cursor_y.saturating_add(1);
                            }
                            if self.cursor_y
                                == self.viewport_height() as u16 - self.scrolling_padding
                            {
                                if !cannot_move_down {
                                    self.viewport_top = self.viewport_top.saturating_add(1);
                                }
                            }
                        }
                    }
                    Action::MoveRight => {
                        self.cursor_x = self.cursor_x.saturating_add(1);
                    }
                    Action::MoveLeft => {
                        self.cursor_x = self.cursor_x.saturating_sub(1);
                        if self.cursor_x < self.viewport_left {
                            self.cursor_x = self.viewport_left;
                        }
                    }

                    Action::Typing(v) => {
                        self.handle_changing(v)?;
                    }
                    Action::EnterKey => {
                        self.handle_enter()?;
                    }
                    Action::TabKey => {
                        self.handle_tab()?;
                    }
                    Action::Backspace => {
                        self.handle_backspace()?;
                    }
                }
            }
        }

        Ok(())
    }

    fn handle_tab(&mut self) -> anyhow::Result<()> {
        match self.mode {
            Mode::Insert => {
                let mut new_line = String::new();
                let editable_line_index = (self.cursor_y + self.viewport_top) as usize;
                let old_line = self.buffer.lines[editable_line_index].clone();

                if self.cursor_x > old_line.len() as u16 {
                    self.stdout.queue(MoveTo(self.cursor_x as u16 + self.buffer.get_file_lenght().to_string().len() as u16+1, self.cursor_y))?;
                    self.cursor_x = old_line.len() as u16;
                }

                if self.cursor_x > 0 {
                    let unchanged_left_part = &old_line[0..self.cursor_x as usize];
                    let unchanged_right_part = &old_line[self.cursor_x as usize..old_line.len()];

                    new_line.push_str(unchanged_left_part);
                    new_line.push_str("    ");
                    new_line.push_str(unchanged_right_part);
                } /*else if self.cursor_x as usize == old_line.len() + 1 {
                    new_line.push_str(&old_line);
                    new_line.push_str("    ");
                } */else {
                    new_line.push_str("    ");
                    new_line.push_str(&old_line);
                }

                //self.stdout.queue(Print(v))?;
                self.buffer.lines[editable_line_index] = new_line;
                self.cursor_x = self.cursor_x.saturating_add(4);
            },
            Mode::Normal => {

            },
            Mode::Command => {

            }
        }

        Ok(())
    }

    fn handle_changing(&mut self, v: char) -> anyhow::Result<()> {
        match self.mode {
            Mode::Insert => {
                let mut new_line = String::new();
                let editable_line_index = (self.cursor_y + self.viewport_top) as usize;
                let old_line = self.buffer.lines[editable_line_index].clone();

                if self.cursor_x > old_line.len() as u16 {
                    self.stdout.queue(MoveTo(self.cursor_x as u16 + self.buffer.get_file_lenght().to_string().len() as u16+1, self.cursor_y))?;
                    self.cursor_x = old_line.len() as u16;
                }

                if self.cursor_x > 0 {
                    let unchanged_left_part = &old_line[0..self.cursor_x as usize];
                    let unchanged_right_part = &old_line[self.cursor_x as usize..old_line.len()];

                    new_line.push_str(unchanged_left_part);
                    new_line.push(v);
                    new_line.push_str(unchanged_right_part);
                } /*else if self.cursor_x as usize == old_line.len() + 1 {
                    new_line.push_str(&old_line);
                    new_line.push(v);
                    new_line.push('2');
                } */else {
                    new_line.push(v);
                    new_line.push_str(&old_line);
                }

                //self.stdout.queue(Print(v))?;
                self.buffer.lines[editable_line_index] = new_line;
                self.cursor_x = self.cursor_x.saturating_add(1);
            }
            Mode::Command => {
                self.cursor_x = self.command_bar.command.len() as u16 + 1;
                // add char to command in command mode
                self.command_bar.command.push(v);
            }
            _ => {}
        }

        Ok(())
    }

    fn handle_backspace(&mut self) -> anyhow::Result<()> {
        match self.mode {
            Mode::Command => {
                if self.command_bar.command.len() > 1 {
                    self.stdout
                        .queue(MoveTo(self.cursor_x.saturating_sub(1), self.cursor_y))?;
                    self.stdout.queue(PrintStyledContent(" ".on(Color::Rgb {
                        r: 255,
                        g: 255,
                        b: 255,
                    })))?;
                    self.cursor_x = self.cursor_x.saturating_sub(1);
                    self.stdout.flush()?;

                    self.command_bar.command.pop();
                }
            }
            Mode::Insert => {
                let mut new_line = String::new();
                let editable_line_index = (self.cursor_y + self.viewport_top) as usize;
                let old_line = self.buffer.lines[editable_line_index].clone();

                if self.cursor_x > old_line.len() as u16 {
                    self.stdout.queue(MoveTo(self.cursor_x as u16 + self.buffer.get_file_lenght().to_string().len() as u16+1, self.cursor_y))?;
                    self.cursor_x = old_line.len() as u16;
                }

                if self.cursor_x > 0 {
                    let unchanged_left_part = &old_line[0..self.cursor_x as usize - 1];
                    let unchanged_right_part = &old_line[self.cursor_x as usize..old_line.len()];
                    new_line.push_str(unchanged_left_part);
                    new_line.push_str(unchanged_right_part);

                    self.buffer.lines[editable_line_index] = new_line;
                    self.cursor_x = self.cursor_x.saturating_sub(1);
                } /*else if self.cursor_x as usize == old_line.len() + 1 {
                    new_line.push_str(&old_line[0..self.cursor_x as usize]);

                    self.buffer.lines[editable_line_index] = new_line;
                    self.cursor_x = self.cursor_x.saturating_sub(1);
                
                } */else {
                    if self.cursor_y > 0 {
                        let previous_line = self.buffer.lines[editable_line_index-1].clone();
                        new_line.push_str(&previous_line);
                        new_line.push_str(&old_line);

                        let l = (&previous_line).len();

                        self.buffer.lines[editable_line_index-1] = new_line;
                        self.cursor_y = self.cursor_y.saturating_sub(1);
                        self.cursor_x = l as u16;
                        self.buffer.lines.remove(editable_line_index);
                    }
                    
                }
            }
            _ => {}
        }

        Ok(())
    }

    fn handle_enter(&mut self) -> anyhow::Result<()>{
        match self.mode {
            Mode::Command => {
                self.execute_command(self.command_bar.command.clone())?;
                self.command_bar.command = String::new();
                self.command_bar.command.push(':');
            }
            Mode::Insert => {
                let editable_line_index = (self.cursor_y + self.viewport_top) as usize;//индекс
                let old_line = self.buffer.lines[editable_line_index].clone(); //старая строка
                if self.cursor_x+1 > old_line.len() as u16{
                    self.buffer.lines.
                    splice(editable_line_index+1..editable_line_index+1, (&["".to_string()])
                    .iter()
                    .cloned());
                    self.cursor_y = self.cursor_y.saturating_add(1);
                    self.cursor_x = 0;
                }
            
                else {// not crossed the boundings
                    let unchanged_left_part = &old_line[0..self.cursor_x as usize];//запоминаем левую часть
                    let unchanged_right_part = &old_line[self.cursor_x as usize..old_line.len()]; //запоминаем правую часть
                    let y = &[unchanged_left_part.to_string(), unchanged_right_part.to_string()];//записываем их в список
                    self.buffer.lines.remove(editable_line_index);//удаляем текущую строку
                    self.buffer.lines.splice(editable_line_index..editable_line_index, y
                        .iter()
                        .cloned());//вставляем те части в середину
                    self.cursor_y = self.cursor_y.saturating_add(1);
                    self.cursor_x = 0;
                }
                
            }
            Mode::Normal => {
                self.cursor_y = self.cursor_y.saturating_add(1);
            }
        }
        Ok(())
    }

    fn execute_command(&mut self, command: String) -> anyhow::Result<()> {
        let _command = command.trim().to_string();
        if _command == ":q".to_string() {
            self.running = false;
        }
        else if _command == ":w".to_string() {
            self.buffer.save()?;
        }
        else if _command.starts_with(":w") {
            let c = _command.clone().trim().to_string();
            let splitted_command = c.split(' ');
            let splitted_command_vec: Vec<&str> = splitted_command.collect();
            let new_file_name = splitted_command_vec.last();

            if let Some(nfn) = new_file_name {
                self.buffer.save_by_name(nfn)?;
            }
        }
        else if _command.starts_with(":e") {
            let c = _command.clone().trim().to_string();
            let splitted_command = c.split(' ');
            let splitted_command_vec: Vec<&str> = splitted_command.collect();
            let new_file_name = splitted_command_vec.last();

            self.buffer.load_file(new_file_name.copied())?;
        }
        
        Ok(())
    }

    fn draw(&mut self, size: (u16, u16)) -> anyhow::Result<()> {
        
        self.draw_viewport()?;
        self.draw_status_bar()?;
        match self.mode {
            Mode::Command => {
                self.draw_command_bar()?;
                return Ok(());
            }
            _ => {
                self.command_bar.clean(&mut self.stdout, size)?;
                return Ok(());
            }
        }
    }

    pub fn viewport_line(&self, n: u16) -> Option<String> {
        let buffer_line = self.viewport_top + n;

        self.buffer.get(buffer_line as usize)
    }

    pub fn number_line(&self, number: u16) -> Option<String>{
        Some((self.viewport_top + number+1).to_string())
    }

    pub fn draw_viewport(&mut self) -> anyhow::Result<()> {
        let file_len = self.buffer.get_file_lenght();
        for i in 0..self.viewport_height() {
            let number_line = self.number_line(i as u16);
            let mut line = String::from(" ".repeat(file_len.to_string().len()+1));
            self.stdout
                .queue(MoveTo(0, i as u16))?
                .queue(Print(line.clone()))?;
                                //условие, если курсор ниже, чем написанный текст в файле
            if i < file_len || i as i32 - 1 < self.cursor_y.into() && self.mode != Mode::Command{
                self.stdout
                    .queue(MoveTo(0, i as u16))?
                    .queue(Print(format!("{}",number_line.clone().unwrap())))?;
            }
            if i < file_len {
                line = match self.viewport_line(i as u16) {
                    Some(s) => s,
                    None => String::new(),
                };
            }
            let w = self.viewport_width();
            self.stdout
                .queue(MoveTo(file_len.to_string().len() as u16+1, i as u16))?
                .queue(Print(format!("{line:<width$}", width = w as usize)))?;
        }
        //self.stdout.queue(MoveTo(0, self.size.1-2))?;
        Ok(())
    }

    pub fn draw_status_bar(&mut self) -> anyhow::Result<()> {
        if self.enable_status_bar {
            self.status_bar.draw(&mut self.stdout, self.size)?;
        }

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

    fn handel_event(&self, ev: event::Event) -> anyhow::Result<Option<Action>> {
        match self.mode {
            Mode::Normal => self.handle_normal_event(ev),
            Mode::Insert => self.handle_insert_event(ev),
            Mode::Command => self.handle_command_event(ev),
        }
    }

    fn handle_normal_event(&self, ev: event::Event) -> anyhow::Result<Option<Action>> {
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
        let _ = self.stdout.execute(terminal::LeaveAlternateScreen); // Leave upper terminal layer
        let _ = terminal::disable_raw_mode();

        self.timer.end();

        let duration_sec = self.timer.get_duration_sec();
        println!("~{} took", duration_sec);
    }
}
