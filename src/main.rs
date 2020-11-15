mod align;
mod attempter;
mod chunk;
mod hand;
mod practice_set;

use crate::{attempter::Attempter, chunk::Chunk, practice_set::PracticeSet};
use std::{fmt, time::Instant};

fn main() {
    print!(
        "{clear}{start}",
        clear = termion::clear::All,
        start = termion::cursor::Goto(1, 1)
    );

    let practice_set = PracticeSet::load().expect("failed to load practice set");

    println!("Type the characters shown after the pipe. Press <Esc> when done\n");

    let word_handler = Chunk::new(practice_set, 20);
    let mut attempter = Attempter::new(word_handler);
    let mut stats = Stats::new();

    println!(
        "{save}{hide}\r{bold}WPM{li}: ----\t{bold}Accuracy{li}: ----",
        bold = termion::style::Bold,
        li = termion::style::Reset,
        hide = termion::cursor::Hide,
        save = "\x1B7",
    );

    loop {
        match attempter.run() {
            Result::Ok(a) => stats.add_attempts(a),
            Result::Err(_) => break,
        }

        println!(
            "{restore}{clear}{stats}",
            restore = "\x1B8",
            stats = stats,
            clear = termion::clear::AfterCursor
        );
    }

    println!("{show}", show = termion::cursor::Show);
}

struct Stats {
    start: Instant,
    typed: usize,
    attempts: usize,
}

impl Stats {
    fn new() -> Self {
        Stats {
            start: Instant::now(),
            typed: 0,
            attempts: 0,
        }
    }

    fn add_attempts(&mut self, attempts: usize) {
        self.typed += 1;
        self.attempts += attempts;
    }
}

impl fmt::Display for Stats {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let typed = self.typed as f64;
        let attempts = self.attempts as f64;

        let duration = Instant::now() - self.start;
        let minutes = (duration.as_millis() as f64) / (60_f64 * 1000_f64);
        let wpm = (typed / 5_f64) / minutes;
        let accuracy = (typed * 100_f64) / attempts;

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
