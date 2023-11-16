use std::io::{self, Write};

pub struct TermBuffer {
    buffer: Vec<u8>,
    fg_color: u8,
}

// Structure for minimizing terminal output operations
impl TermBuffer {
    pub fn new() -> Self {
        Self {
            buffer: vec![],
            fg_color: 0,
        }
    }

    pub fn write(&mut self, data: &[u8]) {
        self.extend(data);
    }

    pub fn flush(&mut self) {
        io::stdout().write(&self.buffer).unwrap();
        io::stdout().flush().unwrap();
        self.buffer.clear();
    }

    fn extend(&mut self, data: &[u8]) {
        self.buffer.extend_from_slice(data);
    }

    pub fn clear_screen(&mut self) {
        self.extend(b"\x1b[2J");
    }

    pub fn move_cursor(&mut self, row: usize, col: usize) {
        self.extend(b"\x1b[");
        self.extend(row.to_string().as_bytes());
        self.extend(b";");
        self.extend(col.to_string().as_bytes());
        self.extend(b"H");
    }

    fn graphic_rendition(&mut self, n: u8) {
        self.extend(b"\x1b[");
        self.extend(n.to_string().as_bytes());
        self.extend(b"m");
    }

    pub fn swap_fg_and_bg_colors(&mut self) {
        self.graphic_rendition(7);
    }

    pub fn reset_appearance(&mut self) {
        self.graphic_rendition(0);
    }

    pub fn set_fg_color(&mut self, color: u8) {
        if self.fg_color != color {
            self.graphic_rendition(color);
            self.fg_color = color;
        }
    }
}
