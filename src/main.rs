use anyhow;
use core::{editor::Editor, buffer::Buffer};

    // mods
mod core;

fn main() -> anyhow::Result<()> {
    let file = std::env::args().nth(1);
    let buffer = Buffer::from_file(file);
    
    let mut editor = Editor::new(buffer).unwrap();
    editor.start()?;

    Ok(())
}

