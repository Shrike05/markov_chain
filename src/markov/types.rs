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
        let mut r = 0_usize;
        for word in &words {
            i.insert(word.clone(), r);
            y.insert(r, word.clone());
            r += 1;
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
}

fn multiply_matrix_vector(matrix: &Vec<f32>, vector: &Vec<f32>) -> Vec<f32> {
    // Result will be a vector of size N
    let n = vector.len();
    let mut result = Vec::with_capacity(n);

    // Iterate through each row of the NxN matrix
    for row_idx in 0..n {
        let start = row_idx * n;
        let end = start + n;
        let row = &matrix[start..end];

        // Compute the dot product of the row and the column vector
        let dot_product: f32 = row
            .iter()
            .zip(vector.iter())
            .map(|(m_val, v_val)| m_val * v_val)
            .sum();

        result.push(dot_product);
    }

    result
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

fn multiply_matrices(
    a: &Vec<f32>,
    shape_a: (usize, usize),
    b: &Vec<f32>,
    shape_b: (usize, usize),
) -> Result<(Vec<f32>, (usize, usize)), String> {
    let (rows_a, cols_a) = shape_a;
    let (rows_b, cols_b) = shape_b;

    // Matrix multiplication requirement: inner dimensions must match
    if cols_a != rows_b {
        return Err(format!(
            "Dimension mismatch: Cannot multiply {}x{} by {}x{}",
            rows_a, cols_a, rows_b, cols_b
        ));
    }

    let mut result = vec![0.0; rows_a * cols_b];

    for i in 0..rows_a {
        for j in 0..cols_b {
            let mut sum = 0.0;
            for k in 0..cols_a {
                // a[i, k] * b[k, j]
                let a_val = a[i * cols_a + k];
                let b_val = b[k * cols_b + j];
                sum += a_val * b_val;
            }
            result[i * cols_b + j] = sum;
        }
    }

    Ok((result, (rows_a, cols_b)))
}

fn count_occurrences<T>(vec: Vec<T>) -> HashMap<T, usize>
where
    T: std::hash::Hash + Eq,
{
    let mut counts = HashMap::new();

    for item in vec {
        // The entry API handles checking if the key exists
        // .or_insert(0) returns a mutable reference to the value
        *counts.entry(item).or_insert(0) += 1;
    }

    counts
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
