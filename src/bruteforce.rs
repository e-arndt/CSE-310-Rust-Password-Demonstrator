use std::time::Instant;

use crate::config::CHARSET;
use crate::hashing::hash_password;
use crate::models::CrackResult;

// Converts a numeric index into a password guess.
// This works like counting in a custom base where each "digit" is a character
// from the selected charset. This lets the program generate guesses without
// storing a list of previous attempts.
fn index_to_guess(mut index: u64, length: usize, charset: &[u8]) -> String {
    let base = charset.len() as u64;
    let mut buffer = vec![charset[0]; length];

    for position in (0..length).rev() {
        let char_index = (index % base) as usize;
        buffer[position] = charset[char_index];
        index /= base;
    }

    String::from_utf8(buffer).expect("Generated guess should be valid UTF-8")
}

// Runs a deterministic brute-force search against the target hash.
// The search starts with shorter passwords first and then increases length,
// which helps demonstrate why short passwords are easier to brute force.
pub fn brute_force(target_hash: &str, max_length: usize) -> Option<CrackResult> {
    let start = Instant::now();
    let mut attempts: u64 = 0;

    for length in 1..=max_length {
        let combinations = CHARSET.len().pow(length as u32) as u64;

        for index in 0..combinations {
            let guess = index_to_guess(index, length, CHARSET);
            attempts += 1;

            // Hash each generated guess and compare the hash to the target hash.
            let guess_hash = hash_password(&guess);

            if guess_hash == target_hash {
                let elapsed = start.elapsed().as_secs_f64();

                return Some(CrackResult {
                    password: guess,
                    attempts,
                    elapsed_seconds: elapsed,
                });
            }
        }
    }

    None
}