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

    for typed in 0.. {
        match attempter.run() {
            Result::Ok(a) => {
                attempts += a;
            }
            Result::Err(_) => {
                print!(
                    "{show}{up}{clear}",
                    show = termion::cursor::Show,
                    clear = termion::clear::CurrentLine,
                    up = termion::cursor::Up(2),
                );

                if attempts == 0 {
                    println!("No results");
                    break;
                }

                let duration = Instant::now() - start;
                let minutes = (duration.as_millis() as f64) / (60.0 * 1000.0);
                let wpm = (typed as f64 / 5.0) / minutes;
                let accuracy = (typed * 100) / attempts;

                println!(
                    "{bold}WPM{reset}: {wpm:.2}\n\r{bold}Accuracy{reset}: {accuracy}%",
                    bold = termion::style::Bold,
                    reset = termion::style::Reset,
                    wpm = wpm,
                    accuracy = accuracy
                );

                break;
            }
        }
    }
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

        write!(self.stdout, "{}\r", goal).unwrap();
        self.stdout.flush().expect("bug: flush");

        for (attempts, k) in io::stdin().keys().enumerate() {
            match k.unwrap() {
                Key::Esc => {
                    write!(self.stdout, "{clear}", clear = termion::clear::CurrentLine)
                        .expect("bug");
                    self.stdout.flush().expect("bug: flush");
                    return Result::Err(());
                }
                Key::Char(k) if k == goal => return Result::Ok(attempts + 1),
                _ => {}
            }
        }

        unreachable!("bug: reached outside of key-event loop")
    }
}
