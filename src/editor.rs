////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////aaaaaaaaaaaaaaaaaaaa
use std::fs;

use super::terminal::settings::Window;
use super::*;
use modes::normal_mode::QuitError;
use terminal::display::write;

mod modes;

struct EditorLine {
    content: Vec<u8>,
    render: Vec<u8>,
}

impl EditorLine {
    fn map_fx_to_rx(&self, fx: usize) -> usize {
        self.content
            .iter()
            .map(|&c| if c == b'\t' { 4 } else { 1 })
            .take(fx)
            .sum::<usize>()
            .saturating_sub(1)
    }
}

enum Mode {
    Normal,
    Insert,
    Command,
}

struct Cursor {
    fx: usize, // file x
    fy: usize, // file y
    rx: usize, // render x
}

struct Offset {
    x: usize, // render units
    y: usize,
}

pub struct Editor {
    file_name: String,
    window: Window,
    cursor: Cursor,
    offset: Offset,
    mode: Mode,
    lines: Vec<EditorLine>,
    cmd_message: String,
    dirty: bool,
}

impl Editor {
    pub fn new(file_path: String) -> Self {
        let mut window = terminal::settings::get_window_size();
        window.num_rows -= 2;

        Self {
            file_name: file_path.clone(),
            window,
            cursor: Cursor {
                fx: 0,
                fy: 0,
                rx: 0,
            },
            offset: Offset { x: 0, y: 0 },
            mode: Mode::Normal,
            lines: read_file(&file_path),
            cmd_message: "You are a great programmer!".to_string(),
            dirty: false,
        }
    }

    fn map_fx_to_rx(&self, rx: usize) -> usize {
        self.lines[self.cursor.fy].map_fx_to_rx(rx)
    }

    pub fn refresh_screen(&mut self) {
        terminal::display::clear_screen();

        self.scroll();
        self.draw_rows();
        self.draw_status_line();
        self.draw_command_line();

        terminal::display::move_cursor(
            self.cursor.fy - self.offset.y + 1,
            self.map_fx_to_rx(self.cursor.fx + 1) - self.offset.x + 1,
        );
    }

    fn scroll(&mut self) {
        self.cursor.rx = self.map_fx_to_rx(self.cursor.fx);

        if self.cursor.fy < self.offset.y {
            self.offset.y = self.cursor.fy;
        }
        if self.cursor.fy >= self.offset.y + self.window.num_rows {
            self.offset.y = self.cursor.fy - self.window.num_rows + 1;
        }
        if self.cursor.rx < self.offset.x {
            self.offset.x = self.cursor.rx;
        }
        if self.cursor.rx >= self.offset.x + self.window.num_cols {
            self.offset.x = self.cursor.rx - self.window.num_cols + 1;
        }
    }

    fn draw_rows(&self) {
        terminal::display::move_cursor(1, 1);

        for i in 0..self.window.num_rows {
            let row = i + self.offset.y;

            if row < self.lines.len() {
                if self.lines[row].render.len() > self.offset.x {
                    let line = &self.lines[row].render[self.offset.x..];
                    write(&line[..self.window.num_cols.min(line.len())]);
                }
            } else {
                write(b"~");
            }

            write(b"\r\n");
        }
    }

    fn draw_status_line(&self) {
        terminal::display::swap_fg_and_bg_colors();

        let line = format!("{:<1$}", &self.file_name, self.window.num_cols);
        write(&line.as_bytes().iter().cloned().collect::<Vec<u8>>());

        terminal::display::reset_appearance();
    }

    fn draw_command_line(&self) {
        let line = format!("{:<1$}", &self.cmd_message, self.window.num_cols);
        write(&line.as_bytes().iter().cloned().collect::<Vec<u8>>());
    }

    pub fn process_key_press(&mut self) -> Result<(), QuitError> {
        let c = terminal::input::read_key();

        match self.mode {
            Mode::Normal => modes::normal_mode::process_key_press(self, c)?,
            Mode::Insert => modes::insert_mode::process_key_press(self, c),
            _ => {}
        }

        Ok(())
    }

    fn insert_char(&mut self, c: u8) {
        let x = self.cursor.fx;
        let y = self.cursor.fy;

        self.lines[y].content.insert(x, c);
        self.lines[y] = build_editor_line(&self.lines[y].content);
        self.cursor.fx += 1;

        self.dirty = true;
    }

    fn delete_char(&mut self) {
        let x = self.cursor.fx;
        let y = self.cursor.fy;
        assert!(x > 0);

        self.lines[y].content.remove(x - 1);
        self.lines[y] = build_editor_line(&self.lines[y].content);
        self.cursor.fx -= 1;

        self.dirty = true;
    }

    fn break_line(&mut self) {
        let x = self.cursor.fx;
        let y = self.cursor.fy;

        let new_line = &self.lines[y].content[x..];
        self.lines.insert(y + 1, build_editor_line(new_line));
        self.lines[y] = build_editor_line(&self.lines[y].content[0..x]);

        self.cursor.fx = 0;
        self.cursor.fy += 1;

        self.dirty = true;
    }

    fn join_lines(&mut self) {
        let x = self.cursor.fx;
        let y = self.cursor.fy;
        assert_eq!(x, 0);
        assert!(y > 0);

        self.cursor.fx = self.lines[y - 1].content.len();
        self.cursor.fy -= 1;

        let mut moved = self.lines[y].content.clone();
        self.lines[y - 1].content.append(&mut moved);
        self.lines[y - 1] = build_editor_line(&self.lines[y - 1].content);
        self.lines.remove(y);

        self.dirty = true;
    }
}

fn read_file(file_path: &str) -> Vec<EditorLine> {
    let mut res: Vec<EditorLine> = fs::read_to_string(file_path)
        .unwrap()
        .split("\n")
        .map(|line| build_editor_line(&line.as_bytes().to_vec()))
        .collect();

    res.pop();
    res
}

fn build_editor_line(content: &[u8]) -> EditorLine {
    let mut render = vec![];
    for &c in content {
        if c == b'\t' {
            for _ in 0..4 {
                render.push(b' ');
            }
        } else {
            render.push(c);
        }
    }

    EditorLine {
        content: content.to_vec(),
        render,
    }
}
