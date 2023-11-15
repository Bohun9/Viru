use crate::editor::Editor;

pub fn forward_search(editor: &mut Editor, pattern: &str) -> bool {
    for i in 0..editor.lines.len() {
        let y = (editor.cursor.fy + i) % editor.lines.len();

        if let Some(x) = search_forward_in_line(
            editor,
            y,
            if i == 0 { editor.cursor.fx } else { 0 },
            pattern,
        ) {
            editor.cursor.fy = y;
            editor.cursor.fx = x;
            return true;
        }
    }

    false
}

pub fn backward_search(editor: &mut Editor, pattern: &str) -> bool {
    let len = editor.lines.len();

    for i in 0..len {
        let y = (editor.cursor.fy + len - i) % len;

        if let Some(x) = search_backward_in_line(
            editor,
            y,
            if i == 0 {
                editor.cursor.fx
            } else {
                editor.lines[y].content.len()
            },
            pattern,
        ) {
            editor.cursor.fy = y;
            editor.cursor.fx = x;
            return true;
        }
    }

    false
}

fn search_forward_in_line(editor: &Editor, y: usize, x: usize, pattern: &str) -> Option<usize> {
    if let Some(x1) = editor.lines[y].content[x..].find(pattern) {
        Some(x1 + x)
    } else {
        None
    }
}

fn search_backward_in_line(editor: &Editor, y: usize, x: usize, pattern: &str) -> Option<usize> {
    editor.lines[y].content[0..x].find(pattern)
}
