use std::io::{self, Read, Write};

fn ctrl_key(c: char) -> u8 {
    (c as u8) % 32
}

pub enum Key {
    Colon,
    Slash,
    Enter,
    Escape,
    Backspace,
    Control(char),
    Char(char),
}

pub fn read_key() -> Key {
    let mut buf: [u8; 1] = [0; 1];

    loop {
        let nread = io::stdin().read(&mut buf[..]).unwrap();
        if nread == 1 {
            break;
        }
    }

    let c = buf[0] as char;
    if c.is_ascii_control() {
        write!(io::stdout(), "key pressed: {}\r\n", c as u8).unwrap();
    } else {
        write!(io::stdout(), "key pressed: {} ('{}')\r\n", c as u8, c).unwrap();
    }

    if buf[0] == b'\x1b' {
        return Key::Escape;
    }

    if buf[0] == b'\x7F' {
        return Key::Backspace;
    }

    if buf[0] == b'\r' {
        return Key::Enter;
    }

    if c == '/' {
        return Key::Slash;
    }

    if c == ':' {
        return Key::Colon;
    }

    for c in 'a'..='z' {
        if buf[0] == ctrl_key(c) {
            return Key::Control(c);
        }
    }

    Key::Char(buf[0] as char)
}
