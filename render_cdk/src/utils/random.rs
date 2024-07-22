#![allow(non_snake_case)]

use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};

pub fn GENERATE_RANDOM_STRING(length: usize) -> String {
    thread_rng()
        .sample_iter(&Alphanumeric)
        .take(length)
        .map(char::from)
        .collect()
}
