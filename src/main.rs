#![allow(dead_code)]
#![allow(unused_variables)]

use std::env;
use std::error::Error;

mod editor;
mod terminal;

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("usage: {} <file>", args[0]);
        std::process::exit(1);
    }

    let fixer = terminal::settings::TerminalFixer::new();
    terminal::settings::enable_row_mode();

    let mut editor = editor::Editor::new(args[1].clone())?;
    loop {
        editor.refresh_screen();

        if let Err(_) = editor.process_key_press() {
            break;
        }
    }

    Ok(())
}
