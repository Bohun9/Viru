use format_bytes::format_bytes;
use std::io::{self, Write};

// Handles flushing standard library buffer
pub fn write(buf: &[u8]) {
    io::stdout().write(buf).unwrap();
    io::stdout().flush().unwrap();
}

pub fn clear_screen() {
    write(b"\x1b[2J");
}

pub fn move_cursor(row: usize, col: usize) {
    write(&format_bytes!(b"\x1b[{};{}H", &(row as u8), &(col as u8)));
}

pub fn swap_fg_and_bg_colors() {
    write(b"\x1b[7m");
}

pub fn reset_appearance() {
    write(b"\x1b[0m");
}
