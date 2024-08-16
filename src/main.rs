use anyhow;
use crossterm::cursor::MoveTo;//добавил
use crate::core::editor::handel_event;//добавил
use core::{editor::Editor, time::Timer, editor::Mode, editor::Action};

use std::{f64::consts::E, io::{stdout, Write}};
use crossterm::{event::read, terminal, ExecutableCommand, QueueableCommand};

// mods
mod core;

fn main() -> anyhow::Result<()> {

    let mut stdout = stdout();
    let mut mode = Mode::Normal;
    let mut editor = Editor::new();
    let mut timer = Timer::new();


    timer.start();

    terminal::enable_raw_mode()?;
    stdout.execute(terminal::EnterAlternateScreen)?; // Enter to the upper terminal layer 
    stdout.execute(terminal::Clear(terminal::ClearType::All))?; // Clear new terminal layer

    // Logic
    //editor.draw(&stdout);
    loop{
        stdout.queue(MoveTo(editor.cursor_x.into(),editor.cursor_y.into()))?;//курсор в начало
        stdout.flush()?;//очистка ввода(всё,что было в stdout)

        if let Some(action) = handel_event(&mode, read()?)?{
            match action {
                Action::Quit => break,
                _=>{}
    
            }
        }
        /*match read()? {//режим считывания
            crossterm::event::Event::Key(event) => match event.code {
            crossterm::event::KeyCode::Char('q') => break,//при нажатии на q выходим из программы
            _=>{}
        },
        _ => {}
    }*/
    }
    

    stdout.execute(terminal::LeaveAlternateScreen)?; // Leave upper terminal layer
    terminal::disable_raw_mode()?;

    timer.end();
    let duration_sec = timer.get_duration_sec();
    println!("~{} took", duration_sec);
    Ok(())
}
