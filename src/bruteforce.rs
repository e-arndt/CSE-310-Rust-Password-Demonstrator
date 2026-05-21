use std::time::Instant;

use crate::hashing::hash_password;
use crate::models::CrackResult;

const CHARSET: &[u8] = b"abcdefghijklmnopqrstuvwxyz";

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

pub fn brute_force(target_hash: &str, max_length: usize) -> Option<CrackResult> {
    let start = Instant::now();
    let mut attempts: u64 = 0;

    for length in 1..=max_length {
        let combinations = CHARSET.len().pow(length as u32) as u64;

        for index in 0..combinations {
            let guess = index_to_guess(index, length, CHARSET);
            attempts += 1;

            if attempts % 100_000 == 0 {
                let elapsed = start.elapsed().as_secs_f64();
                let rate = attempts as f64 / elapsed;

                println!(
                    "Attempts: {} | Current guess: {} | Rate: {:.0} guesses/sec",
                    attempts, guess, rate
                );
            }

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