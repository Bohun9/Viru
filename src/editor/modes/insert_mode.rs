use super::super::Mode;
use crate::editor::Editor;
use crate::terminal::input::Key;

pub fn process_key_press(editor: &mut Editor, key: Key) {
    match key {
        Key::Escape => {
            editor.mode = Mode::Normal;
        }
        Key::Enter => {
            editor.break_line();
        }
        Key::Char(c) => editor.insert_char(c as u8),
        Key::Backspace => {
            if editor.cursor.fx > 0 {
                editor.delete_previous_char()
            } else if editor.cursor.fy > 0 {
                editor.join_lines();
            }
        }
        _ => {}
    }
}
