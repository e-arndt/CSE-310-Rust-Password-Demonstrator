use std::io::{self, Write};
use std::process::Command;

mod bruteforce;
mod config;
mod hashing;
mod models;

use bruteforce::brute_force;
use config::{CHARSET_LABEL, MAX_LENGTH};
use hashing::hash_password;


// Validates the user's password before running the brute-force demo.
// These limits keep the project educational, controlled, and fast enough
// for a short classroom demonstration.
fn validate_target(target: &str, max_length: usize) -> Result<(), String> {
    if target.is_empty() {
        return Err("Password cannot be empty.".to_string());
    }

    if target.len() > max_length {
        return Err(format!(
            "Password is too long. Maximum length is {}.",
            max_length
        ));
    }

    if !target.chars().all(|c| c.is_ascii_lowercase()) {
        return Err("Password must only contain lowercase letters a-z.".to_string());
    }

    Ok(())
}

fn clear_screen() {
    if cfg!(target_os = "windows") {
        Command::new("cmd")
            .args(["/C", "cls"])
            .status()
            .expect("Failed to clear screen");
    } else {
        Command::new("clear")
            .status()
            .expect("Failed to clear screen");
    }
}

fn main() {
    let mut error_message: Option<String> = None;

    let target = loop {
        clear_screen();

        println!("RustPassLab");
        println!("Educational brute-force password strength demonstrator");
        println!("Target password length limit: {}", MAX_LENGTH);
        println!("Charset: {}", CHARSET_LABEL);
        println!();

        if let Some(message) = &error_message {
            println!("Error: {}", message);
            println!("Please try again.\n");
        }

        let mut input = String::new();

        print!(
            "Enter a lowercase password to test (max length {}): ",
            MAX_LENGTH
        );
        io::stdout().flush().expect("Failed to flush stdout");

        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read input");

        let input = input.trim().to_string();

        match validate_target(&input, MAX_LENGTH) {
            Ok(()) => break input,
            Err(message) => {
                error_message = Some(message);
            }
        }
    };

    clear_screen();

    println!("RustPassLab");
    println!("Educational brute-force password strength demonstrator");
    println!("Target password length limit: {}", MAX_LENGTH);
    println!("Charset: {}", CHARSET_LABEL);
    println!();

    let target_hash = hash_password(&target);

    println!("Target SHA-256 hash: {}", target_hash);
    println!();

    match brute_force(&target_hash, MAX_LENGTH) {
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