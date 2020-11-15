use std::{fmt, time::Instant};

pub enum Stats {
    Empty,
    Filled(Filled),
}

impl Stats {
    pub fn new() -> Self {
        Stats::Empty
    }

    pub fn add_attempts(&mut self, attempts: usize) {
        match self {
            Stats::Empty => *self = Stats::Filled(Filled::new(attempts)),
            Stats::Filled(f) => f.add(attempts),
        }
    }
}

impl fmt::Display for Stats {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let (wpm, accuracy) = match self {
            Stats::Empty => (0_f64, 0_f64),
            Stats::Filled(f) => (f.wpm(), f.accuracy()),
        };

        write!(
            f,
            "{bold}WPM{reset}: {wpm:.2}\t{bold}Accuracy{reset}: {accuracy:.2}%",
            bold = termion::style::Bold,
            reset = termion::style::Reset,
            wpm = wpm,
            accuracy = accuracy
        )
    }
}

pub struct Filled {
    start: Instant,
    end: Instant,
    typed: usize,
    attempts: usize,
}

const CHARS_PER_WORD: f64 = 5_f64;
const MILLIS_PER_MINUTE: f64 = 60_f64 * 1000_f64;

impl Filled {
    fn new(attempts: usize) -> Self {
        let now = Instant::now();

        Filled {
            typed: 1,
            start: now,
            end: now,
            attempts,
        }
    }

    fn add(&mut self, attempts: usize) {
        self.attempts += attempts;
        self.typed += 1;
        self.end = Instant::now();
    }

    fn wpm(&self) -> f64 {
        if self.start == self.end {
            0_f64
        } else {
            let minutes = ((self.end - self.start).as_millis() as f64) / MILLIS_PER_MINUTE;
            (self.typed as f64 / CHARS_PER_WORD) / minutes
        }
    }

    fn accuracy(&self) -> f64 {
        (self.typed as f64) / (self.attempts as f64) * 100_f64
    }
}
