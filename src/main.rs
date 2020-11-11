use rand::{rngs::ThreadRng, seq::IteratorRandom as _};
use std::{
    io::{self, Stdout, Write as _},
    time::Instant,
};
use termion::{event::Key, input::TermRead as _, raw::IntoRawMode as _, raw::RawTerminal};

fn main() {
    println!(
        "{}{}Type the character under the cursor. Press <Esc> when done\n",
        termion::clear::All,
        termion::cursor::Goto(1, 1),
    );

    let mut attempter = Attempter::new("aren".to_string());

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
    stdout: RawTerminal<Stdout>,
    rng: ThreadRng,
    options: String,
    chunk: [char; 20],
}

impl Attempter {
    fn new(options: String) -> Self {
        let mut rng = rand::thread_rng();
        let mut chunk = ['0'; 20];
        (0..chunk.len()).for_each(|i| {
            chunk[i] = options.chars().choose(&mut rng).unwrap();
        });

        Attempter {
            options,
            chunk,
            rng,
            stdout: io::stdout().into_raw_mode().unwrap(),
        }
    }

    fn run(&mut self) -> Result<usize, ()> {
        let chunk: String = self.chunk.iter().collect();
        let goal = self.chunk[0];
        (1..chunk.len()).for_each(|i| {
            self.chunk[i - 1] = self.chunk[i];
        });
        self.chunk[self.chunk.len() - 1] = self.options.chars().choose(&mut self.rng).unwrap();

        let center = termion::terminal_size().unwrap().0 / 2;

        write!(
            self.stdout,
            "\r{clear}{center}{save}{chunk}{left}\n\n
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
            left = termion::cursor::Left(self.chunk.len() as u16),
            lp = termion::color::Fg(termion::color::Cyan),
            lr = termion::color::Fg(termion::color::Red),
            lm = termion::color::Fg(termion::color::Blue),
            li = termion::color::Fg(termion::color::LightRed),
            ri = termion::color::Fg(termion::color::Yellow),
            rm = termion::color::Fg(termion::color::LightGreen),
            rr = termion::color::Fg(termion::color::Green),
            rp = termion::color::Fg(termion::color::LightBlue),
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
