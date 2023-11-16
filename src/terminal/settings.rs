use super::*;
use std::io::{self, Read};
use std::os::unix::io::AsRawFd;
use termios::*;

fn get_terminal_settings() -> Termios {
    let stdout_fd = io::stdout().as_raw_fd();
    let termios = Termios::from_fd(stdout_fd).unwrap();
    return termios;
}

pub struct TerminalFixer {
    orig_termios: Termios,
}

impl TerminalFixer {
    pub fn new() -> Self {
        TerminalFixer {
            orig_termios: get_terminal_settings(),
        }
    }
}

impl Drop for TerminalFixer {
    fn drop(&mut self) {
        let mut term_buf = display::TermBuffer::new();
        term_buf.clear_screen();
        term_buf.move_cursor(1, 1);
        term_buf.flush();

        let stdout_fd = io::stdout().as_raw_fd();
        tcsetattr(stdout_fd, TCSAFLUSH, &self.orig_termios).unwrap();
    }
}

pub fn enable_row_mode() {
    let stdout_fd = io::stdout().as_raw_fd();
    let mut termios = get_terminal_settings();

    termios.c_iflag &= !(ICRNL | IXON);
    termios.c_oflag &= !(OPOST);
    termios.c_lflag &= !(ECHO | ICANON | ISIG | IEXTEN);
    termios.c_cc[VMIN] = 0;
    termios.c_cc[VTIME] = 1;

    tcsetattr(stdout_fd, TCSAFLUSH, &termios).unwrap();
}

pub struct Window {
    pub num_rows: usize,
    pub num_cols: usize,
}

pub fn get_window_size() -> Window {
    let mut term_buf = display::TermBuffer::new();
    term_buf.write(b"\x1b[666B");
    term_buf.write(b"\x1b[666C");
    term_buf.write(b"\x1b[6n");
    term_buf.flush();

    let mut buf: [u8; 64] = [0; 64];
    let mut len = 0;

    loop {
        assert!(len < buf.len());
        io::stdin().read(&mut buf[len..=len]).unwrap();

        if buf[len] == b'R' {
            buf[len] = b'0';
            break;
        }
        len += 1;
    }

    assert_eq!(buf[0], b'\x1b');
    assert_eq!(buf[1], b'[');

    let s = std::str::from_utf8(&buf[2..len]).unwrap();
    let mut parts = s.split(";");

    let num_rows = parts.next().unwrap().parse().unwrap();
    let num_cols = parts.next().unwrap().parse().unwrap();

    return Window { num_rows, num_cols };
}
