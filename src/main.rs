mod practice_set;

use crate::practice_set::PracticeSet;
use practice_set::Finger;
use std::{
    io::{self, Write as _},
    time::Instant,
};
use termion::{event::Key, input::TermRead as _, raw::IntoRawMode as _, raw::RawTerminal};

fn main() {
    print!("{clear}", clear = termion::clear::All);

    let practice_set = PracticeSet::load().expect("failed to load practice set");

    println!(
        "{}Type the character under the cursor. Press <Esc> when done\n",
        termion::cursor::Goto(1, 1),
    );

    let mut attempter = Attempter::new(practice_set);

    let mut attempts = 0;
    let start = Instant::now();

    println!(
        "\r{bold}WPM{li}: ----\t{bold}Accuracy{li}: ----",
        bold = termion::style::Bold,
        li = termion::style::Reset,
    );

    for typed in 1.. {
        match attempter.run() {
            Result::Ok(a) => attempts += a,
            Result::Err(_) => break,
        }

        update_stats(start, typed, attempts);
    }

    print!("{}", termion::cursor::Show);
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

struct Attempter {
    stdout: RawTerminal<io::Stdout>,
    practice_set: PracticeSet,
    chunk: Vec<(char, Finger)>,
}

impl Attempter {
    fn new(mut practice_set: PracticeSet) -> Self {
        let chunk = practice_set.choose_n(20);

        Attempter {
            chunk,
            practice_set,
            stdout: io::stdout().into_raw_mode().unwrap(),
        }
    }

    fn run(&mut self) -> Result<usize, ()> {
        let chunk: String = self.chunk.iter().map(|(c, _)| c).collect();
        let (goal, finger) = self.chunk[0];
        (1..chunk.len()).for_each(|i| {
            self.chunk[i - 1] = self.chunk[i];
        });
        let last_index = self.chunk.len() - 1;
        self.chunk[last_index] = self.practice_set.choose();

        let center = termion::terminal_size().unwrap().0 / 2;

        let finger_color = |displayed| {
            if displayed == finger {
                termion::color::Fg(termion::color::Red).to_string()
            } else {
                termion::style::Reset.to_string()
            }
        };

        /*
            .-.                     .-.
          .-| |-.                 .-| |-.
          | | | |                 | | | |
        .-| | | |                 | | | |-.
        | | | | |                 | | | | |
        | | | | |-.             .-| | | | |
        | '     | |             | |     ` |
        |       | |             | |       |
        |         |             |         |
        \         /             \         /
         |       |               |       |
         |       |               |       |
           */

        write!(
            self.stdout,
            "\r{clear}{center}{save}{chunk}\n\n
\r{center_hand}{xx}  {xx}  {lm}.-{lm}.{xx}                   {xx}  {rm}.-{rm}.{xx}
\r{center_hand}{xx}  {lr}.-{lm}| {lm}|{li}-.                 {ri}.-{rm}| {rm}|{rr}-{rr}.{xx}
\r{center_hand}{xx}  {lr}| {lr}| {li}|{li} |                 {ri}| {ri}| {rr}|{rr} {rr}|{xx}
\r{center_hand}{lp}.-{xx}| {xx}| {xx}|{xx} |                 {xx}| {xx}| {xx}|{xx} {xx}|{rp}-.
\r{center_hand}{lp}| {lp}| {xx}| {xx}|{xx} |                 {xx}| {xx}| {xx}|{xx} {rp}|{rp} |
\r{center_hand}{xx}| {xx}| {xx}| {xx}|{xx} |-.             .-{xx}| {xx}| {xx}|{xx} {xx}|{xx} |
\r{center_hand}{xx}| {xx}' {xx}  {xx} {xx} | |             | {xx}| {xx}  {xx} {xx} {xx}`{xx} |
\r{center_hand}{xx}| {xx}  {xx}  {xx} {xx} | |             | {xx}| {xx}  {xx} {xx} {xx} {xx} |
\r{center_hand}{xx}| {xx}  {xx}  {xx} {xx}   |             | {xx}  {xx}  {xx} {xx} {xx} {xx} |
\r{center_hand}{xx}\\{xx}  {xx}  {xx} {xx}    /             \\{xx} {xx}  {xx} {xx} {xx} {xx}   /
\r{center_hand}{xx} |{xx}  {xx}  {xx} {xx}  |               |{xx}  {xx}  {xx} {xx} {xx} {xx}|
\r{center_hand}{xx} |{xx}  {xx}  {xx} {xx}  |               |{xx}  {xx}  {xx} {xx} {xx} {xx}|{restore}",
            clear = termion::clear::AfterCursor,
            chunk = chunk,
            center = termion::cursor::Right(center),
            center_hand = termion::cursor::Right(center - 18 + (chunk.len() / 2) as u16),
            lp = finger_color(Finger::LeftPinky),
            lr = finger_color(Finger::LeftRing),
            lm = finger_color(Finger::LeftMiddle),
            li = finger_color(Finger::LeftIndex),
            ri = finger_color(Finger::RightIndex),
            rm = finger_color(Finger::RightMiddle),
            rr = finger_color(Finger::RightRing),
            rp = finger_color(Finger::RightPinky),
            xx = termion::style::Reset,
            save = termion::cursor::Save,
            restore = termion::cursor::Restore,
        )
        .unwrap();

        self.stdout.flush().expect("bug: flush");

        for (attempts, k) in io::stdin().keys().enumerate() {
            match k.unwrap() {
                Key::Esc => {
                    write!(self.stdout, "{clear}", clear = termion::clear::AfterCursor)
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
