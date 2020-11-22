mod align;
mod attempter;
mod chunk;
mod hand;
mod practice_set;

use std::{
    io::{self, Write},
    thread,
    time::Duration,
    time::Instant,
};

use crate::{
    attempter::Attempter,
    chunk::Chunk,
    hand::Hand,
    practice_set::{Finger, PracticeData, PracticeSet},
};

enum Event {
    Elapsed(Duration),
    Typed(usize),
    Updated {
        finger: Option<Finger>,
        words: String,
    },
    Ended,
}

const CHARS_PER_WORD: f64 = 5_f64;
const SECS_PER_MINUTE: f64 = 60_f64;
const PRACTICE_TIME: Duration = Duration::from_secs(120);

fn main() {
    print!(
        "{clear}{start}",
        clear = termion::clear::All,
        start = termion::cursor::Goto(1, 1)
    );

    let practice_data = PracticeData::load().expect("failed to load practice set");

    let (tx, rx) = std::sync::mpsc::channel::<Event>();
    let timer_sender = tx.clone();

    thread::spawn(move || {
        let practice_set = PracticeSet::new(practice_data);
        let mut chunk = Chunk::new(practice_set, 20);
        let attempter = Attempter::new();

        loop {
            let words = chunk.to_string();
            let (goal, finger) = chunk.next();
            tx.send(Event::Updated { finger, words }).unwrap();

            match attempter.attempt(goal) {
                attempter::State::Exit => break,
                attempter::State::Continue(attempts) => tx.send(Event::Typed(attempts)).unwrap(),
            };
        }

        tx.send(Event::Ended)
    });

    thread::spawn(move || {
        let mut start = Instant::now();

        loop {
            thread::sleep(Duration::from_millis(20));
            let now = Instant::now();
            timer_sender.send(Event::Elapsed(now - start)).unwrap();
            start = now;
        }
    });

    let width = termion::terminal_size().unwrap().0;
    let mut hand = Hand::new(align::Left((width - Hand::WIDTH) / 2));
    let mut words = "".to_string();
    let mut remaining = PRACTICE_TIME;
    let mut typed = 0;
    let mut attempts = 0;

    print!(
        "Type the characters shown after the pipe. Press <Esc> when done\n\n\r{hide}{save}",
        hide = termion::cursor::Hide,
        save = "\x1B7"
    );

    io::stdout().flush().unwrap();

    let align = align::Left(width / 2);
    for event in rx.into_iter() {
        match event {
            Event::Ended => break,
            Event::Elapsed(elapsed) => match remaining.checked_sub(elapsed) {
                Some(difference) => remaining = difference,
                None => {
                    remaining = Duration::default();
                    break;
                }
            },
            Event::Updated { words: w, finger } => {
                words = w;
                hand.select(finger)
            }
            Event::Typed(a) => {
                typed += 1;
                attempts += a;
            }
        }

        let timer = format!(
            "{minutes}:{seconds:.3}",
            minutes = remaining.as_secs() / 60,
            seconds = remaining.as_secs_f64() % 60_f64
        );

        println!(
            "{restore}{clear} {timer}

{align}|{words}

{hand}",
            restore = "\x1B8",
            clear = termion::clear::AfterCursor,
            align = align,
            words = words,
            hand = hand,
            timer = timer,
        );
    }

    print!("\n\r");

    if typed < 1 {
        println!("At least try one, c'mon!")
    } else {
        let typed = typed as f64;
        let minutes = (PRACTICE_TIME - remaining).as_secs_f64() / SECS_PER_MINUTE;
        let wpm = (typed / CHARS_PER_WORD) / minutes;
        let accuracy = typed / (attempts as f64) * 100_f64;

        println!(
            "{bold}WPM{reset}: {wpm:.2}\t{bold}Accuracy{reset}: {accuracy:.2}%",
            bold = termion::style::Bold,
            reset = termion::style::Reset,
            wpm = wpm,
            accuracy = accuracy,
        );
    }

    print!("{show}", show = termion::cursor::Show);
}
