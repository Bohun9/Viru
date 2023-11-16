use super::super::Mode;
use super::super::*;
use super::*;
use crate::editor::Editor;
use crate::terminal::input::Key;

pub struct QuitError {}

fn normalize_fx(editor: &mut Editor) {
    editor.cursor.fx = editor.cursor.fx.min(
        editor.lines[editor.cursor.fy]
            .content
            .len()
            .saturating_sub(1),
    );
}

fn move_cursor(editor: &mut Editor, key: Key) {
    match key {
        Key::Char('h') => {
            if editor.cursor.fx > 0 {
                editor.cursor.fx -= 1;
            }
        }
        Key::Char('j') => {
            if editor.cursor.fy + 1 < editor.lines.len() {
                editor.cursor.fy += 1;
            }
        }
        Key::Char('k') => {
            if editor.cursor.fy > 0 {
                editor.cursor.fy -= 1;
            }
        }
        Key::Char('l') => {
            if editor.cursor.fx + 1 < editor.lines[editor.cursor.fy].render.len() {
                editor.cursor.fx += 1;
            }
        }
        _ => {
            panic!("internal error");
        }
    }

    normalize_fx(editor);
}

pub fn process_key_press(editor: &mut Editor, key: Key) -> Result<(), QuitError> {
    let orig_cursor = editor.cursor.clone();

    match key {
        Key::Char(k) if ['h', 'j', 'k', 'l'].contains(&k) => {
            move_cursor(editor, key);
        }
        Key::Char('0') => {
            editor.cursor.fx = 0;
        }
        Key::Char('$') => {
            editor.cursor.fx = editor.lines[editor.cursor.fy]
                .render
                .len()
                .saturating_sub(1)
        }
        Key::Char('i') => {
            editor.mode = Mode::INSERT;
        }
        Key::Char('I') => {
            editor.cursor.fx = 0;
            editor.mode = Mode::INSERT;
        }
        Key::Char('a') => {
            editor.mode = Mode::INSERT;
            editor.cursor.fx += 1;
        }
        Key::Char('A') => {
            editor.mode = Mode::INSERT;
            editor.cursor.fx = editor.lines[editor.cursor.fy].content.len();
        }
        Key::Char('o') => {
            editor.mode = Mode::INSERT;
            editor.add_blank_line(editor.cursor.fy + 1);
            editor.cursor.fx = 0;
            editor.cursor.fy += 1;
        }
        Key::Char('x') => {
            editor.delete_current_char();
        }
        Key::Control('d') => {
            editor.cursor.fy = (editor.cursor.fy + 30).min(editor.lines.len().saturating_sub(1));
            normalize_fx(editor);
        }
        Key::Control('u') => {
            editor.cursor.fy = editor.cursor.fy.saturating_sub(30);
            normalize_fx(editor);
        }
        Key::Slash => {
            let maybe_pattern =
                command_mode::enter_command(editor, "/", Some(searching::forward_search));

            if let Some(pattern) = maybe_pattern {
                editor.last_pattern = Some(pattern);
            } else {
                editor.cursor = orig_cursor;
            }
        }
        Key::Char('n') => {
            if let Some(pattern) = editor.last_pattern.clone() {
                move_cursor(editor, Key::Char('l'));
                if !searching::forward_search(editor, &pattern) {
                    editor.cursor = orig_cursor;
                }
            }
        }
        Key::Char('N') => {
            if let Some(pattern) = editor.last_pattern.clone() {
                move_cursor(editor, Key::Char('h'));
                if !searching::backward_search(editor, &pattern) {
                    editor.cursor = orig_cursor;
                }
            }
        }
        Key::Colon => {
            let maybe_command = command_mode::enter_command(editor, ":", None);
            if let Some(command) = maybe_command {
                if command == "q" {
                    return Err(QuitError {});
                }
                if command == "w" {
                    editor.save_file();
                }
            }
        }
        _ => {}
    }

    Ok(())
}
