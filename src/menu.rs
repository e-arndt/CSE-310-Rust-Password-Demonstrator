use std::io::{self, Write};
use std::process::Command;
use num_format::{Locale, ToFormattedString};

use crate::models::CrackResult;
use crate::strong_estimator::{format_duration, StrongEstimate};

pub struct PasswordMode {
    pub title: &'static str,
    pub prompt: &'static str,
    pub charset_label: &'static str,
    pub max_length: usize,
    pub charset: &'static [u8],
    pub validator: fn(char) -> bool,
    pub validation_message: &'static str,
    pub result_label: &'static str,
}

pub fn clear_screen() {
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

fn print_app_header() {
    println!("================= Password Strength Demonstrator =================");
    println!();
}

fn validate_target(target: &str, mode: &PasswordMode) -> Result<(), String> {
    if target.is_empty() {
        return Err("Password cannot be empty.".to_string());
    }

    if target.len() > mode.max_length {
        return Err(format!(
            "Password is too long. Maximum length is {}.",
            mode.max_length
        ));
    }

    if !target.chars().all(mode.validator) {
        return Err(mode.validation_message.to_string());
    }

    Ok(())
}

fn print_mode_details(mode: &PasswordMode) {
    println!("{}", mode.title);
    println!();
    println!("Target password length limit: {}", mode.max_length);
    println!("Charset: {}", mode.charset_label);
    println!();
}

pub fn read_password_for_mode(mode: &PasswordMode) -> String {
    let mut error_message: Option<String> = None;

    loop {
        clear_screen();
        print_app_header();
        print_mode_details(mode);

        if let Some(message) = &error_message {
            println!("Error: {}", message);
            println!("Please try again.");
            println!();
        }

        let mut input = String::new();

        print!("{}", mode.prompt);
        io::stdout().flush().expect("Failed to flush stdout");

        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read input");

        let input = input.trim().to_string();

        match validate_target(&input, mode) {
            Ok(()) => {
                clear_screen();
                return input;
            }
            Err(message) => {
                error_message = Some(message);
            }
        }
    }
}

pub fn print_bruteforce_progress_screen(_mode: &PasswordMode) {
    clear_screen();
    print_app_header();

    println!("Brute-force in progress...");
    println!();
}

pub fn print_crack_result(
    mode: &PasswordMode,
    target_hash: &str,
    result: &CrackResult,
) -> f64 {
    clear_screen();
    let rate = result.guesses_per_second();
    print_app_header();

    println!("Target Password SHA-256 Hash: {}", target_hash);
    println!("**- Matched -** SHA-256 Hash: {}", result.matched_hash);
    println!();

    println!("{}", mode.result_label);
    println!("Password: {}", result.password);
    println!(
    "Attempts: {}",
        result.attempts.to_formatted_string(&Locale::en)
    );
    println!("Elapsed time: {:.4} seconds", result.elapsed_seconds);
    println!(
    "Average rate: {} attempts/sec",
        (rate as u64).to_formatted_string(&Locale::en)
    );

    rate
}

pub fn print_password_not_found() {
    println!("Password was not found within the configured search space.");
}

pub fn print_strong_estimate_screen(charset_label: &str, local_average_rate: f64) {
    clear_screen();
    print_app_header();

    println!("Strong password estimate");
    println!("Charset: {}", charset_label);
    println!("Local measured rate: {:.0} guesses/sec", local_average_rate);
    println!();
}

pub fn print_strong_estimate_result(estimate: &StrongEstimate) {
    println!("Password analyzed: {}", estimate.password);
    println!("Length: {}", estimate.password.len());
    println!("Charset size: {}", estimate.charset_size);
    println!("Estimated attempts: {}", estimate.estimated_attempts);
    println!(
        "Estimated brute-force time on this PC: {}",
        format_duration(estimate.estimated_seconds)
    );
    println!();
    println!();
}

pub fn print_estimate_failed() {
    println!("Unable to estimate this password.");
}

pub fn pause_for_enter() {
    println!();
    println!("Press Enter to continue...");

    let mut pause = String::new();

    io::stdin()
        .read_line(&mut pause)
        .expect("Failed to pause program");
}