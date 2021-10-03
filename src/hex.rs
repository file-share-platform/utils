//! Handles all things hex for generation
use rand::{thread_rng, Rng};

/// Randomly generate any specified length of hex chars, returned in the form of a String.
pub fn get_random_hex(length: usize) -> String {
    thread_rng()
        .sample_iter(rand::distributions::Alphanumeric)
        .take(length)
        .map(char::from)
        .collect()
}
