use super::super::Mode;
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
}

pub fn process_key_press(editor: &mut Editor, key: Key) -> Result<(), QuitError> {
    match key {
        Key::Control('q') => {
            return Err(QuitError {});
        }
        Key::Char(k) if ['h', 'j', 'k', 'l'].contains(&k) => {
            move_cursor(editor, key);
            normalize_fx(editor);
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
            editor.mode = Mode::Insert;
        }
        Key::Char('a') => {
            editor.mode = Mode::Insert;
            editor.cursor.fx += 1;
        }
        Key::Control('d') => {
            editor.cursor.fy = (editor.cursor.fy + 30).min(editor.lines.len().saturating_sub(1));
            normalize_fx(editor);
        }
        Key::Control('u') => {
            editor.cursor.fy = editor.cursor.fy.saturating_sub(30);
            normalize_fx(editor);
        }
        _ => {}
    }

    Ok(())
}
