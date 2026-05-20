use std::time::Instant;
use std::io::{self, Write};
use sha2::{Digest, Sha256};

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

fn hash_password(input: &str) -> String {
    let mut hasher = Sha256::new();

    hasher.update(input.as_bytes());

    let result = hasher.finalize();

    format!("{:x}", result)
}

struct CrackResult {
    password: String,
    attempts: u64,
    elapsed_seconds: f64,
}

impl CrackResult {
    fn guesses_per_second(&self) -> f64 {
        self.attempts as f64 / self.elapsed_seconds
    }
}

fn brute_force(target_hash: &str, max_length: usize) -> Option<CrackResult> {
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

fn main() {
    let max_length = 5;
    let mut target = String::new();

    println!("RustPassLab");
    println!("Educational brute-force password strength demonstrator");
    println!("Target password length limit: {}", max_length);
    println!("Charset: lowercase a-z");
    println!();

    print!("Enter a lowercase password to test (max length {}): ", max_length);
    io::stdout().flush().expect("Failed to flush stdout");

    io::stdin()
        .read_line(&mut target)
        .expect("Failed to read input");

    let target = target.trim().to_string();

    if target.is_empty() {
        println!("Password cannot be empty.");
        return;
    }

    if target.len() > max_length {
        println!("Password is too long. Maximum length is {}.", max_length);
        return;
    }

    if !target.chars().all(|c| c.is_ascii_lowercase()) {
        println!("Password must only contain lowercase letters a-z.");
        return;
    }

    let target_hash = hash_password(&target);

    println!("Target SHA-256 hash: {}", target_hash);
    println!();


    match brute_force(&target_hash, max_length) {
        Some(result) => {
            println!();
            println!("Password found!");
            println!("Password: {}", result.password);
            println!("Attempts: {}", result.attempts);
            println!("Elapsed time: {:.4} seconds", result.elapsed_seconds);
            println!(
                "Average rate: {:.0} guesses/sec",
                result.guesses_per_second()
            );
        }
        None => {
            println!("Password was not found within the configured search space.");
        }
    }
}