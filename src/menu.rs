// ================================
// Imports
// Standard input/output tools, terminal commands, formatting helpers, and project models.
// ================================

use std::io::{self, Write};
use std::process::Command;
use num_format::{Locale, ToFormattedString};

use crate::models::CrackResult;
use crate::strong_estimator::{format_duration, StrongEstimate};

// ================================
// Password mode configuration
// Defines the labels, limits, character sets, and validation rules for each CLI mode.
// ================================

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

// ================================
// Terminal display helpers
// Clears the screen and prints shared CLI header information.
// ================================

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

// ================================
// Input validation
// Checks user-entered passwords against the selected demo mode rules.
// ================================

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

// ================================
// Mode information display
// Prints the title, password length limit, and character set for the selected mode.
// ================================

fn print_mode_details(mode: &PasswordMode) {
    println!("{}", mode.title);
    println!();
    println!("Target password length limit: {}", mode.max_length);
    println!("Charset: {}", mode.charset_label);
    println!();
}

// ================================
// Password input workflow
// Repeats input prompts until the user enters a password valid for the selected mode.
// ================================

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

// ================================
// Brute-force progress display
// Shows the CLI screen while a brute-force demo is running.
// ================================

pub fn print_bruteforce_progress_screen(_mode: &PasswordMode) {
    clear_screen();
    print_app_header();

    println!("Brute-force in progress...");
    println!();
}

// ================================
// Brute-force result display
// Prints matched hash details, discovered password, attempts, elapsed time, and local rate.
// ================================

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

// ================================
// Not-found message
// Prints the fallback message when a password is outside the configured search space.
// ================================

pub fn print_password_not_found() {
    println!("Password was not found within the configured search space.");
}

// ================================
// Strong estimate intro display
// Prints the strong password estimate heading, charset, and measured local rate.
// ================================

pub fn print_strong_estimate_screen(charset_label: &str, local_average_rate: f64) {
    clear_screen();
    print_app_header();

    println!("Strong password estimate");
    println!("Charset: {}", charset_label);
    println!("Local measured rate: {:.0} guesses/sec", local_average_rate);
    println!();
}

// ================================
// Strong estimate result display
// Prints average-case brute-force estimate details for the strong password mode.
// ================================

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

// ================================
// Estimate failure message
// Prints the fallback message when a strong password estimate cannot be calculated.
// ================================

pub fn print_estimate_failed() {
    println!("Unable to estimate this password.");
}

// ================================
// Pause prompt
// Waits for the user to press Enter before continuing the CLI flow.
// ================================

pub fn pause_for_enter() {
    println!();
    println!("Press Enter to continue...");

    let mut pause = String::new();

    io::stdin()
        .read_line(&mut pause)
        .expect("Failed to pause program");
}