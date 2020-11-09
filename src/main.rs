use rand::seq::IteratorRandom as _;
use std::io::{self, Write as _};
use termion::{event::Key, input::TermRead as _, raw::IntoRawMode as _};

fn main() {
    let mut stdout = io::stdout().into_raw_mode().unwrap();

    let row = ['a', 's', 'd', 'f', 'g', 'h', 'j', 'k', 'l'];
    let mut rng = rand::thread_rng();

    write!(
        stdout,
        "{}{}Type the character being presented. Press <Esc> when done\n\n{}",
        termion::clear::All,
        termion::cursor::Goto(1, 1),
        termion::cursor::Goto(1, 3),
    )
    .expect("bug: write");

    stdout.flush().expect("bug: flush");

    'outer: loop {
        let stdin = io::stdin();
        let c = *row.iter().choose(&mut rng).expect("bug: empty home row");
        write!(
            stdout,
            "{}{}{}",
            termion::cursor::Goto(1, 3),
            termion::clear::CurrentLine,
            c
        )
        .expect("bug: write");
        stdout.flush().expect("bug: flush");

        'inner: for k in stdin.keys() {
            match k.unwrap() {
                Key::Esc => break 'outer,
                Key::Char(typed) => {
                    if typed == c {
                        stdout.flush().expect("bug: flush");
                        break 'inner;
                    }
                }
                _ => {}
            }

            stdout.flush().expect("bug: flush");
        }
    }

    write!(stdout, "{}", termion::cursor::Show).unwrap();
}
