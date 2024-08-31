use std::{borrow::BorrowMut, io::{Stdout, Write}};

use crossterm::{cursor::MoveTo, style::{self, Color, Stylize}, QueueableCommand};

// adding all list of modules
use super::{modules::{get_modules, BarModule}, Editor, Mode};

use super::config::{CommandBarSettings,CommandsBindings};

pub struct CommandBar {
    background_color: (u8, u8, u8),
    pub command: String,
    set_design: CommandBarSettings,
    set: CommandsBindings,
}

impl CommandBar {
    pub fn new() -> Self {
        Self {
            set_design: CommandBarSettings {
                background_color: (255, 255, 255),
                color: (0,0,0),
            },
            set: CommandsBindings { quit: String::from(":q"), save: String::from(":w"), edit: String::from(":e") },
            command: ":".to_string(),
            background_color: (255, 255, 255)
        }
    }

    // draw status bar and all modules in the terminal
    pub fn draw(&mut self, _stdout: &mut Stdout, size: (u16, u16)) -> anyhow::Result<()> {
        _stdout.queue(MoveTo(0, size.1))?;
        _stdout.queue(style::PrintStyledContent(
            (" ").repeat((size.0) as usize).on(
                Color::Rgb {
                    r: self.background_color.0, 
                    g: self.background_color.1, 
                    b: self.background_color.2 
                })
        ))?;
        _stdout.flush()?;
        _stdout.queue(MoveTo(0, size.1))?;

        _stdout.queue(style::PrintStyledContent(
            self.command.as_str().on(
                Color::Rgb { 
                    r: self.background_color.0,
                    g: self.background_color.1, 
                    b: self.background_color.2 
                })
                .with(Color::Rgb { 
                    r: 102, 
                    g: 0, 
                    b: 51 
                })
            ))?;
        _stdout.flush()?;
        Ok(())
    }

    pub fn clean(&mut self, _stdout: &mut Stdout, size: (u16, u16)) -> anyhow::Result<()> {
        _stdout.queue(MoveTo(0, size.1))?;
        _stdout.queue(style::Print(
            (" ").repeat((size.0) as usize)
        ))?;
        _stdout.flush()?;

        _stdout.queue(MoveTo(0, size.1))?;
        _stdout.flush()?;
        _stdout.queue(MoveTo(0, size.1))?;
        Ok(())
    }
    // changes background color of status bar
    pub fn change_background_color(&mut self, new_color: &(u8, u8, u8)) {
        self.background_color = new_color.clone();
    }

    // moves cursor to the begining of status bar
    fn move_to_bar(&self, _stdout: &mut Stdout, size: (u16, u16)) -> anyhow::Result<()> {
        _stdout.queue(MoveTo(0, size.1-1))?;
        Ok(())
    }
}
