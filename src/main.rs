mod hand;
mod practice_set;

use crate::{hand::Hand, practice_set::PracticeSet};
use std::{
    collections::VecDeque,
    io::{self, Write as _},
    time::Instant,
};
use termion::{event::Key, input::TermRead as _, raw::IntoRawMode as _, raw::RawTerminal};

fn main() {
    print!(
        "{clear}{start}",
        clear = termion::clear::All,
        start = termion::cursor::Goto(1, 1)
    );

    let practice_set = PracticeSet::load().expect("failed to load practice set");

    println!("Type the characters shown after the pipe. Press <Esc> when done\n");

    let word_handler = WordHandler::new(practice_set, 20);
    let mut attempter = Attempter::new(word_handler);

    let mut attempts = 0;
    let start = Instant::now();

    println!(
        "{hide}\r{bold}WPM{li}: ----\t{bold}Accuracy{li}: ----",
        bold = termion::style::Bold,
        li = termion::style::Reset,
        hide = termion::cursor::Hide,
    );

    for typed in 1.. {
        match attempter.run() {
            Result::Ok(a) => attempts += a,
            Result::Err(_) => break,
        }

        update_stats(start, typed, attempts);
    }

    print!("{show}", show = termion::cursor::Show);
}

fn update_stats(start: Instant, typed: usize, attempts: usize) {
    let typed = typed as f64;
    let attempts = attempts as f64;

    let duration = Instant::now() - start;
    let minutes = (duration.as_millis() as f64) / (60_f64 * 1000_f64);
    let wpm = (typed / 5_f64) / minutes;
    let accuracy = (typed * 100_f64) / attempts;

    println!(
        "{clear}\r{bold}WPM{li}: {wpm:.2}\t{bold}Accuracy{li}: {accuracy:.2}%",
        clear = termion::clear::CurrentLine,
        bold = termion::style::Bold,
        li = termion::style::Reset,
        wpm = wpm,
        accuracy = accuracy
    );
}

struct WordHandler {
    practice_set: PracticeSet,
    words: VecDeque<VecDeque<char>>,
    window_len: usize,
}

impl WordHandler {
    fn new(mut practice_set: PracticeSet, window_len: usize) -> Self {
        WordHandler {
            words: practice_set
                .choose_n((window_len / 2) + 1)
                .into_iter()
                .map(|word| word.chars().collect())
                .collect(),
            practice_set,
            window_len,
        }
    }

    // Return visible portion of words
    fn chunk(&self) -> String {
        self.words
            .iter()
            .flat_map(|word| word.iter().copied().chain(std::iter::once(' ')))
            .take(self.window_len)
            .collect()
    }

    // Return next character to type
    fn next_char(&mut self) -> char {
        match self.words[0].pop_front() {
            Some(c) => c,
            None => {
                self.words.pop_front();
                let next_word = self.practice_set.choose().chars().collect();
                self.words.push_back(next_word);
                ' '
            }
        }
    }
}

struct Attempter {
    word_handler: WordHandler,
    stdout: RawTerminal<io::Stdout>,
    hand: Hand,
    cursor_pos: u16,
}

impl Attempter {
    fn new(word_handler: WordHandler) -> Self {
        let cursor_pos = termion::terminal_size().unwrap().0 / 2;

        Attempter {
            hand: Hand::new(
                cursor_pos - (word_handler.window_len as u16 - 2)
                    + (word_handler.window_len / 2) as u16,
            ),
            stdout: io::stdout().into_raw_mode().unwrap(),
            cursor_pos,
            word_handler,
        }
    }

    fn run(&mut self) -> Result<usize, ()> {
        let chunk = self.word_handler.chunk();
        let goal = self.word_handler.next_char();

        let finger = self.word_handler.practice_set.finger(goal);
        self.hand.select(finger);

        write!(
            self.stdout,
            "\r{clear}{right}{save}|{chunk}\n\n{hand}{restore}",
            clear = termion::clear::AfterCursor,
            chunk = chunk,
            right = termion::cursor::Right(self.cursor_pos),
            hand = self.hand,
            save = "\x1B7",
            restore = "\x1B8",
        )
        .unwrap();

        self.stdout.flush().expect("bug: flush");

        for (attempts, k) in io::stdin().keys().enumerate() {
            match k.unwrap() {
                Key::Esc => {
                    write!(
                        self.stdout,
                        "{clear}\r",
                        clear = termion::clear::AfterCursor,
                    )
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
