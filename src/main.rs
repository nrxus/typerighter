mod align;
mod attempter;
mod chunk;
mod hand;
mod practice_set;
mod stats;

use std::io::{self, Write};

use crate::{attempter::Attempter, chunk::Chunk, hand::Hand, practice_set::PracticeSet, stats::Stats};

fn main() {
    print!(
        "{clear}{start}",
        clear = termion::clear::All,
        start = termion::cursor::Goto(1, 1)
    );

    let practice_set = PracticeSet::load().expect("failed to load practice set");

    let mut chunk = Chunk::new(practice_set, 20);
    let center = (termion::terminal_size().unwrap().0) / 2;
    let mut hand = Hand::new(center);
    let mut stats = Stats::new();
    let aligner = align::Left(center - 1);
    let attempter = Attempter::new();

    print!(
        "Type the characters shown after the pipe. Press <Esc> when done\n\n\r{hide}{save}",
        hide = termion::cursor::Hide,
        save = "\x1B7"
    );

    io::stdout().flush().unwrap();

    loop {
        println!(
            "{stats}\n{align}|{chunk}\n",
            stats = stats,
            align = aligner,
            chunk = chunk,
        );

        let (goal, finger) = chunk.next();

        hand.select(finger);
        println!("{}", hand);

        let state = attempter.attempt(goal);

        print!(
            "{restore}{clear}",
            restore = "\x1B8",
            clear = termion::clear::AfterCursor
        );

        match state {
            attempter::State::Exit => {
                break;
            }
            attempter::State::Continue(a) => {
                stats.add_attempts(a);
            }
        }
    }

    println!(
        "\r{stats}{show}",
        stats = stats,
        show = termion::cursor::Show
    );
}
