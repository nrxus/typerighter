use crate::{align, chunk::Chunk, hand::Hand};
use std::io::{self, Write as _};
use termion::{
    event::Key,
    input::TermRead as _,
    raw::{IntoRawMode as _, RawTerminal},
};

pub struct Attempter {
    chunk: Chunk,
    stdout: RawTerminal<io::Stdout>,
    hand: Hand,
    aligner: align::Left,
}

impl Attempter {
    pub fn new(chunk: Chunk) -> Self {
        let center = (termion::terminal_size().unwrap().0) / 2;

        Attempter {
            hand: Hand::new(center),
            stdout: io::stdout().into_raw_mode().unwrap(),
            aligner: align::Left(center - 1),
            chunk,
        }
    }

    pub fn run(&mut self) -> Result<usize, ()> {
        let (goal, finger) = self.chunk.next();
        self.hand.select(finger);

        write!(
            self.stdout,
            "{align}|{goal}{chunk}\n\n{hand}",
            chunk = self.chunk,
            align = self.aligner,
            hand = self.hand,
            goal = goal,
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
