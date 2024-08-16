use anyhow;

use core::{editor::Editor, time::Timer};

use std::io::stdout;
use crossterm::{terminal, event::read, ExecutableCommand};

// mods
mod core;

fn main() -> anyhow::Result<()> {

    let mut stdout = stdout();
    let editor = Editor::new();
    let mut timer = Timer::new();
    timer.start();

    terminal::enable_raw_mode()?;
    stdout.execute(terminal::EnterAlternateScreen)?; // Enter to the upper terminal layer 
    stdout.execute(terminal::Clear(terminal::ClearType::All))?; // Clear new terminal layer

    // Logic
    editor.draw(&stdout);


    stdout.execute(terminal::LeaveAlternateScreen)?; // Leave upper terminal layer
    terminal::disable_raw_mode()?;

    timer.end();
    let duration_sec = timer.get_duration_sec();
    println!("~{} took", duration_sec);
    Ok(())
}
