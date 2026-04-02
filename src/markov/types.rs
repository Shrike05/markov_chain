use bevy::{
    platform::collections::{HashMap, HashSet},
    prelude::*,
};
use rand::RngExt;

#[derive(Resource, Clone, Debug, PartialEq)]
pub struct MarkovChain {
    pub words: HashSet<String>,
    word_indicies: HashMap<String, usize>,
    index_word: HashMap<usize, String>,
    pub transitions: Vec<f32>,
}

impl MarkovChain {
    pub fn new(words: HashSet<String>, transitions: Vec<f32>) -> MarkovChain {
        let mut i = HashMap::new();
        let mut y = HashMap::new();
        for (r, word) in words.iter().enumerate() {
            i.insert(word.clone(), r);
            y.insert(r, word.clone());
        }
        MarkovChain {
            words,
            word_indicies: i,
            index_word: y,
            transitions,
        }
    }

    pub fn from_text<T: Into<String>>(txt: T) -> Self {
        let all_words = txt
            .into()
            .split_whitespace()
            .map(|x| x.to_string())
            .collect::<Vec<String>>();
        let words: HashSet<String> = all_words.clone().into_iter().collect();

        let mut i = HashMap::new();
        let mut y = HashMap::new();
        for (r, word) in words.iter().enumerate() {
            i.insert(word.clone(), r);
            y.insert(r, word.clone());
        }

        let transitions = get_transition_matrix(&words, &i, &all_words);

        MarkovChain {
            words,
            word_indicies: i,
            index_word: y,
            transitions,
        }
    }

    pub fn sample(&self, word: String) -> Option<&String> {
        let i = *self.word_i(&word)?;
        let next_word_dist: Vec<f32> = self.transitions[i..self.words.len() + i].to_vec();

        let word_index = sample_index(&next_word_dist);

        self.i_word(word_index)
    }

    pub fn word_to_vec(&self, word: &String) -> Option<Vec<f32>> {
        let mut res = vec![0.; self.words.len()];
        let pos = self.word_i(word)?;
        res[*pos] = 1.;

        Some(res)
    }

    pub fn word_i(&self, word: &String) -> Option<&usize> {
        self.word_indicies.get(word)
    }

    pub fn i_word(&self, i: usize) -> Option<&String> {
        self.index_word.get(&i)
    }

    pub fn transition_from_x_to_y(&self, x: &String, y: &String) -> Option<f32> {
        let x_i = self.word_i(x)?;
        let y_i = self.word_i(y)?;

        let index = x_i * self.words.len() + y_i;
        Some(self.transitions[index])
    }
}

fn sample_index(probs: &[f32]) -> usize {
    let mut rng = rand::rng();
    let r: f32 = rng.random_range(0.0..=1.);

    let mut cumulative_sum = 0.0;
    for (i, &p) in probs.iter().enumerate() {
        cumulative_sum += p;
        if r < cumulative_sum {
            return i;
        }
    }

    // Fallback for rounding errors (e.g., if sum was 0.99999)
    probs.len() - 1
}

fn get_transition_matrix(
    unique_words: &HashSet<String>,
    word_to_i: &HashMap<String, usize>,
    words: &[String], // Use slice for flexibility
) -> Vec<f32> {
    let n = unique_words.len();
    let mut res = vec![0.0; n * n];

    // 1. Single pass to count transitions: O(W)
    // We look at each pair of words (window of 2)
    for window in words.windows(2) {
        let curr_word = &window[0];
        let next_word = &window[1];

        if let (Some(&i), Some(&j)) = (word_to_i.get(curr_word), word_to_i.get(next_word)) {
            res[i * n + j] += 1.0;
        }
    }

    // 2. Normalize rows to get probabilities: O(U^2)
    for i in 0..n {
        let row_start = i * n;
        let row_end = row_start + n;

        // Sum the counts for this specific row (word i)
        let row_sum: f32 = res[row_start..row_end].iter().sum();

        if row_sum > 0.0 {
            for j in 0..n {
                res[row_start + j] /= row_sum;
            }
        }
    }

    res
}
