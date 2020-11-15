use crate::practice_set::{Finger, PracticeSet};
use std::{collections::VecDeque, fmt};

pub struct Chunk {
    practice_set: PracticeSet,
    words: VecDeque<VecDeque<char>>,
    window_len: usize,
}

impl Chunk {
    pub fn new(mut practice_set: PracticeSet, window_len: usize) -> Self {
        Chunk {
            words: practice_set
                .choose_n((window_len / 2) + 1)
                .into_iter()
                .map(|word| word.chars().collect())
                .collect(),
            practice_set,
            window_len,
        }
    }

    // Return next character to type
    pub fn next(&mut self) -> (char, Option<Finger>) {
        let goal = match self.words[0].pop_front() {
            Some(c) => c,
            None => {
                self.words.pop_front();
                let next_word = self.practice_set.choose().chars().collect();
                self.words.push_back(next_word);
                ' '
            }
        };

        (goal, self.practice_set.finger(goal))
    }
}

impl fmt::Display for Chunk {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let chunk: String = self
            .words
            .iter()
            .flat_map(|word| word.iter().copied().chain(std::iter::once(' ')))
            .take(self.window_len)
            .collect();

        write!(f, "{}", chunk)
    }
}
