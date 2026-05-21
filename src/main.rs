use std::io::{self, Write};
use std::process::Command;

mod bruteforce;
mod config;
mod hashing;
mod models;
mod strong_estimator;

use bruteforce::brute_force;
use config::{
    MAX_MODERATE_PASSWORD_LENGTH, MAX_STRONG_PASSWORD_LENGTH, MAX_WEAK_PASSWORD_LENGTH,
    MODERATE_CHARSET, MODERATE_CHARSET_LABEL, STRONG_CHARSET, STRONG_CHARSET_LABEL,
    WEAK_CHARSET, WEAK_CHARSET_LABEL,
};
use hashing::hash_password;
use strong_estimator::{estimate_strong_password, format_duration};

struct PasswordMode {
    title: &'static str,
    prompt: &'static str,
    charset_label: &'static str,
    max_length: usize,
    charset: &'static [u8],
    validator: fn(char) -> bool,
    validation_message: &'static str,
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

fn read_password_for_mode(mode: &PasswordMode) -> String {
    let mut error_message: Option<String> = None;

    loop {
        clear_screen();

        println!("RustPassLab");
        println!("Educational brute-force password strength demonstrator");
        println!();
        println!("{}", mode.title);
        println!("Target password length limit: {}", mode.max_length);
        println!("Charset: {}", mode.charset_label);
        println!();

        if let Some(message) = &error_message {
            println!("Error: {}", message);
            println!("Please try again.\n");
        }

        let mut input = String::new();

        print!("{}", mode.prompt);
        io::stdout().flush().expect("Failed to flush stdout");

        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read input");

        let input = input.trim().to_string();

        match validate_target(&input, mode) {
            Ok(()) => return input,
            Err(message) => {
                error_message = Some(message);
            }
        }
    }
}

fn run_bruteforce_demo(mode: &PasswordMode) -> Option<f64> {
    let target = read_password_for_mode(mode);

    clear_screen();

    println!("RustPassLab");
    println!("Educational brute-force password strength demonstrator");
    println!();
    println!("{}", mode.title);
    println!("Target password length limit: {}", mode.max_length);
    println!("Charset: {}", mode.charset_label);
    println!();

    let target_hash = hash_password(&target);

    println!("Target SHA-256 hash: {}", target_hash);
    println!();

    let rate = match brute_force(&target_hash, mode.max_length, mode.charset) {
        Some(result) => {
            let rate = result.guesses_per_second();

            println!();
            println!("Password found!");
            println!("Password: {}", result.password);
            println!("Attempts: {}", result.attempts);
            println!("Elapsed time: {:.4} seconds", result.elapsed_seconds);
            println!("Average rate: {:.0} guesses/sec", rate);

            Some(rate)
        }
        None => {
            println!("Password was not found within the configured search space.");
            None
        }
    };

    println!();
    println!("Press Enter to continue...");
    let mut pause = String::new();
    io::stdin()
        .read_line(&mut pause)
        .expect("Failed to pause program");

    rate
}

fn main() {
    let weak_mode = PasswordMode {
        title: "Weak password demonstration",
        prompt: "Enter a lowercase password to test: ",
        charset_label: WEAK_CHARSET_LABEL,
        max_length: MAX_WEAK_PASSWORD_LENGTH,
        charset: WEAK_CHARSET,
        validator: |c| c.is_ascii_lowercase(),
        validation_message: "Password must only contain lowercase letters a-z.",
    };

    let moderate_mode = PasswordMode {
        title: "Moderate password demonstration",
        prompt: "Enter a moderate password using letters and numbers: ",
        charset_label: MODERATE_CHARSET_LABEL,
        max_length: MAX_MODERATE_PASSWORD_LENGTH,
        charset: MODERATE_CHARSET,
        validator: |c| c.is_ascii_alphanumeric(),
        validation_message: "Password must only contain letters A-Z, a-z, and digits 0-9.",
    };

    let weak_rate = run_bruteforce_demo(&weak_mode);
    let moderate_rate = run_bruteforce_demo(&moderate_mode);

    let local_average_rate = match (weak_rate, moderate_rate) {
        (Some(weak), Some(moderate)) => (weak + moderate) / 2.0,
        (Some(weak), None) => weak,
        (None, Some(moderate)) => moderate,
        (None, None) => 0.0,
    };

    let strong_password = read_password_for_mode(&PasswordMode {
        title: "Strong password estimate",
        prompt: "Enter a strong password to estimate only: ",
        charset_label: STRONG_CHARSET_LABEL,
        max_length: MAX_STRONG_PASSWORD_LENGTH,
        charset: STRONG_CHARSET,
        validator: |c| STRONG_CHARSET.contains(&(c as u8)),
        validation_message: "Password must only contain supported letters, digits, and symbols.",
    });

    clear_screen();

    println!("RustPassLab");
    println!("Educational brute-force password strength demonstrator");
    println!();
    println!("Strong password estimate");
    println!("Charset: {}", STRONG_CHARSET_LABEL);
    println!("Local measured rate: {:.0} guesses/sec", local_average_rate);
    println!();

    match estimate_strong_password(&strong_password, STRONG_CHARSET, local_average_rate) {
        Some(estimate) => {
            println!("Password analyzed: {}", estimate.password);
            println!("Length: {}", estimate.password.len());
            println!("Charset size: {}", estimate.charset_size);
            println!("Estimated attempts: {}", estimate.estimated_attempts);
            println!(
                "Estimated brute-force time on this PC: {}",
                format_duration(estimate.estimated_seconds)
            );
        }
        None => {
            println!("Unable to estimate this password.");
        }
    }
}