use std::io;
use termion::{event::Key, input::TermRead as _, raw::IntoRawMode as _, raw::RawTerminal};

pub struct Attempter {
    _raw: RawTerminal<io::Stdout>,
}

impl Attempter {
    pub fn new() -> Self {
        Attempter {
            _raw: io::stdout().into_raw_mode().unwrap(),
        }
    }

    pub fn attempt(&self, goal: char) -> State {
        for (attempts, k) in io::stdin().keys().enumerate() {
            match k.unwrap() {
                Key::Esc | Key::Ctrl('c') => return State::Exit,
                Key::Char(k) if k == goal => return State::Continue(attempts + 1),
                _ => {}
            }
        }

        unreachable!("bug: reached outside of key-event loop")
    }
}

pub enum State {
    Exit,
    Continue(usize),
}
