use std::{
    boxed::Box,
    collections::HashMap,
    error::Error,
    fs::File,
    io::{self, Write as _},
};

use rand::{rngs::ThreadRng, seq::IteratorRandom as _};

#[derive(serde::Deserialize, Debug, Clone, Copy, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum Finger {
    LeftPinky,
    LeftRing,
    LeftMiddle,
    LeftIndex,
    RightIndex,
    RightMiddle,
    RightRing,
    RightPinky,
}

#[derive(Debug)]
pub struct PracticeSet {
    keys: HashMap<char, Finger>,
    rng: ThreadRng,
    words: Vec<&'static str>,
}

impl PracticeSet {
    pub fn load() -> Result<Self, Box<dyn Error>> {
        let sets: Vec<KeySet> = {
            let keysets = File::open("./practice_sets.yml")?;
            serde_yaml::from_reader(io::BufReader::new(keysets))?
        };

        user_selection(sets).map(|keys| PracticeSet {
            words: include_str!("../words.txt")
                .lines()
                .filter(|word| word.chars().all(|c| keys.contains_key(&c)))
                .collect(),
            keys,
            rng: ThreadRng::default(),
        })
    }

    pub fn choose(&mut self) -> &'static str {
        assert!(self.words.len() > 0);

        self.words.iter().choose(&mut self.rng).unwrap()
    }

    pub fn choose_n(&mut self, n: usize) -> Vec<&'static str> {
        (0..n).map(|_| self.choose()).collect()
    }

    pub fn finger(&self, c: char) -> Option<Finger> {
        self.keys.get(&c).copied()
    }
}

#[derive(serde::Deserialize, Debug, Clone)]
struct KeySet {
    name: String,
    keys: HashMap<char, Finger>,
}

fn user_selection(sets: Vec<KeySet>) -> Result<HashMap<char, Finger>, Box<dyn Error>> {
    let stdin = io::stdin();
    let mut input = String::new();

    let options = sets
        .iter()
        .enumerate()
        .map(|(i, k)| {
            format!(
                "{bold}[{option}]{reset} {name}",
                option = i,
                name = k.name,
                bold = termion::style::Bold,
                reset = termion::style::Reset
            )
        })
        .collect::<Vec<_>>()
        .join("\n");

    loop {
        print!(
            "Practice Sets\n{options}\n\nSelect a practice set: ",
            options = options
        );

        io::stdout().flush()?;

        input.clear();
        stdin.read_line(&mut input)?;
        match input.trim().parse::<usize>() {
            Ok(c) if c < sets.len() => return Ok(sets[c].keys.clone()),
            _ => {}
        }

        println!(
            "\n{bold}Not a valid selection\n",
            bold = termion::style::Bold
        );
    }
}
