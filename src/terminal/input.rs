use std::io::{self, Read};

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

    // C-M = Enter
    for c in ('a'..='z').filter(|&c| c != 'm').collect::<Vec<char>>() {
        if buf[0] == ctrl_key(c) {
            return Key::Control(c);
        }
    }

    match buf[0] {
        b'\x1B' => Key::Escape,
        b'\x7F' => Key::Backspace,
        b'\r' => Key::Enter,
        b'/' => Key::Slash,
        b':' => Key::Colon,
        u => Key::Char(u as char),
    }
}
