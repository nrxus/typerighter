use rand::{rngs::ThreadRng, seq::IteratorRandom as _};
use std::{
    io::{self, Stdout, Write as _},
    time::Instant,
};
use termion::{event::Key, input::TermRead as _, raw::IntoRawMode as _, raw::RawTerminal};

fn main() {
    print!(
        "Type the character being presented. Press <Esc> when done\n\n{}",
        termion::cursor::Hide,
    );

    let mut attempter = Attempter::new("arenbgsito".to_string());

    let mut attempts = 0;
    let start = Instant::now();

    println!(
        "\r{bold}WPM{reset}: ----\t{bold}Accuracy{reset}: ----",
        bold = termion::style::Bold,
        reset = termion::style::Reset,
    );

    for typed in 1.. {
        match attempter.run() {
            Result::Ok(a) => attempts += a,
            Result::Err(_) => break,
        }

        update_stats(start, typed, attempts);
    }
}

fn update_stats(start: Instant, typed: usize, attempts: usize) {
    let typed = typed as f64;
    let attempts = attempts as f64;

    let duration = Instant::now() - start;
    let minutes = (duration.as_millis() as f64) / (60_f64 * 1000_f64);
    let wpm = (typed / 5_f64) / minutes;
    let accuracy = (typed * 100_f64) / attempts;

    println!(
        "{clear}\r{bold}WPM{reset}: {wpm:.2}\t{bold}Accuracy{reset}: {accuracy:.2}%",
        clear = termion::clear::CurrentLine,
        bold = termion::style::Bold,
        reset = termion::style::Reset,
        wpm = wpm,
        accuracy = accuracy
    );
}

struct Attempter {
    stdout: RawTerminal<Stdout>,
    rng: ThreadRng,
    options: String,
}

impl Attempter {
    fn new(options: String) -> Self {
        Attempter {
            stdout: io::stdout().into_raw_mode().unwrap(),
            options,
            rng: rand::thread_rng(),
        }
    }

    fn run(&mut self) -> Result<usize, ()> {
        let goal = self.options.chars().choose(&mut self.rng).unwrap();

        write!(self.stdout, "\r{}", goal).unwrap();
        self.stdout.flush().expect("bug: flush");

        for (attempts, k) in io::stdin().keys().enumerate() {
            match k.unwrap() {
                Key::Esc => {
                    write!(self.stdout, "{clear}", clear = termion::clear::CurrentLine)
                        .expect("bug");
                    self.stdout.flush().expect("bug: flush");
                    return Result::Err(());
                }
                Key::Char(k) if k == goal => {
                    write!(self.stdout, "{}", termion::cursor::Up(1)).expect("bug");
                    return Result::Ok(attempts + 1);
                }
                _ => {}
            }
        }

        unreachable!("bug: reached outside of key-event loop")
    }
}
