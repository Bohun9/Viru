use super::super::Mode;
use crate::editor::Editor;
use crate::terminal::input::{read_key, Key};

pub fn enter_command(
    editor: &mut Editor,
    prompt_str: &str,
    callback: Option<fn(&mut Editor, &str) -> bool>,
) -> Option<String> {
    let mut prompt = String::new();
    editor.mode = Mode::Command;

    loop {
        editor.cmd_message = prompt_str.to_string() + &prompt;
        editor.refresh_screen();

        let key = read_key();

        match key {
            Key::Enter => {
                editor.mode = Mode::Normal;
                return Some(prompt);
            }
            Key::Escape => {
                editor.mode = Mode::Normal;
                return None;
            }
            Key::Char(c) => prompt.push(c),
            Key::Backspace => {
                if !prompt.is_empty() {
                    prompt.pop();
                }
            }
            _ => {}
        }

        if let Some(c) = callback {
            c(editor, &prompt);
        }
    }
}
