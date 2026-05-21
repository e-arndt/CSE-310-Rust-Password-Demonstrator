use std::io::{self, Write};

mod bruteforce;
mod hashing;
mod models;

use bruteforce::brute_force;
use hashing::hash_password;

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