use std::{borrow::BorrowMut, io::{Stdout, Write}};

use crossterm::{cursor::MoveTo, style::{self, Color, Stylize}, QueueableCommand};

// adding all list of modules
use super::{modules::{get_modules, BarModule}, Editor, Mode};

#[derive(Clone, Copy)]
pub enum Side {
    Top,
    Bottom
}
pub struct StatusBar {
    sepatator: &'static str,
    modules: Vec<Box<dyn BarModule>>,
    side: Side,

    background_color: (u8, u8, u8)
}

impl StatusBar {
    pub fn new() -> Self {
        Self {
            sepatator: "◢◤", 
            modules: get_modules(), 
            side: Side::Bottom,
            background_color: (255, 204, 229)
        }
    }

    // draw status bar and all modules in the terminal
    pub fn draw(&mut self, _stdout: &mut Stdout, size: (u16, u16)) -> anyhow::Result<()> {
        let mut bar: String = String::new();

        for module in self.modules.iter_mut() {
            let displayed_string = module.give_display();
            bar.push_str(&displayed_string.as_str());
            bar.push(' ');
            bar.push_str(self.sepatator);
            bar.push(' ');
        }

        self.move_to_bar(_stdout, size)?;

        _stdout.queue(style::PrintStyledContent(
            " ".repeat(size.0 as usize).on(
                Color::Rgb { 
                    r: self.background_color.0, 
                    g: self.background_color.1, 
                    b: self.background_color.2 
                })
        ))?;
        _stdout.flush()?;

        self.move_to_bar(_stdout, size)?;
        _stdout.queue(style::PrintStyledContent(
            bar.as_str().on(
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

    pub fn change_side(&mut self) { // changes side of the status bar
        match self.side {
            Side::Bottom => self.side = Side::Top,
            Side::Top => self.side = Side::Bottom
        }
    }

    // changes background color of status bar
    pub fn change_background_color(&mut self, new_color: &(u8, u8, u8)) {
        self.background_color = new_color.clone();
    }

    pub fn get_editor_info(&mut self, info: (Mode, (u16, u16))) {
        for module in self.modules.iter_mut(){
            //module.get_editor_info(info);
        }
    }

    // moves cursor to the begining of status bar
    fn move_to_bar(&self, _stdout: &mut Stdout, size: (u16, u16)) -> anyhow::Result<()> {
        match self.side {
            Side::Bottom => {
                _stdout.queue(MoveTo(0, size.1-1))?;
            },
            Side::Top => {
                _stdout.queue(MoveTo(0, 0))?;
            }
        }
        Ok(())
    }
}
