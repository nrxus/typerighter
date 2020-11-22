use std::{collections::VecDeque, fmt};

pub struct TypedChars {
    chars: VecDeque<char>,
    capacity: usize,
}

impl TypedChars {
    pub fn new(capacity: usize) -> Self {
        TypedChars {
            chars: VecDeque::with_capacity(capacity),
            capacity,
        }
    }

    pub fn add(&mut self, a: char) {
        if self.chars.len() == self.capacity {
            self.chars.pop_front();
        }
        self.chars.push_back(a);
    }
}

impl fmt::Display for TypedChars {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use termion::color;

        let chunk: String = self.chars.iter().collect();

        write!(
            f,
            "{color}{chunk:>width$}{reset}",
            color = color::Fg(color::AnsiValue::grayscale(12)),
            chunk = chunk,
            width = self.capacity,
            reset = color::Fg(color::Reset)
        )
    }
}
