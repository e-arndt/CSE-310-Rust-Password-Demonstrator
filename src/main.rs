mod bruteforce;
mod config;
mod hashing;
mod menu;
mod models;
mod strong_estimator;

use bruteforce::brute_force;
use config::{
    MAX_MODERATE_PASSWORD_LENGTH, MAX_STRONG_PASSWORD_LENGTH, MAX_WEAK_PASSWORD_LENGTH,
    MODERATE_CHARSET, MODERATE_CHARSET_LABEL, STRONG_CHARSET, STRONG_CHARSET_LABEL,
    WEAK_CHARSET, WEAK_CHARSET_LABEL,
};
use hashing::hash_password;
use menu::{
    pause_for_enter, print_bruteforce_progress_screen, print_crack_result, print_estimate_failed,
    print_password_not_found, print_strong_estimate_result, print_strong_estimate_screen,
    read_password_for_mode, PasswordMode,
};
use strong_estimator::estimate_strong_password;

fn run_bruteforce_demo(mode: &PasswordMode) -> Option<f64> {
    let target = read_password_for_mode(mode);
    let target_hash = hash_password(&target);

    print_bruteforce_progress_screen(mode, &target_hash);

    let rate = match brute_force(&target_hash, mode.max_length, mode.charset) {
        Some(result) => Some(print_crack_result(mode, &target_hash, &result)),
        None => {
            print_password_not_found();
            None
        }
    };

    pause_for_enter();

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
        result_label: "Weak Password found!",
    };

    let moderate_mode = PasswordMode {
        title: "Moderate password demonstration",
        prompt: "Enter a moderate password using letters and numbers: ",
        charset_label: MODERATE_CHARSET_LABEL,
        max_length: MAX_MODERATE_PASSWORD_LENGTH,
        charset: MODERATE_CHARSET,
        validator: |c| c.is_ascii_alphanumeric(),
        validation_message: "Password must only contain letters A-Z, a-z, and digits 0-9.",
        result_label: "Moderate Password found!",
    };

    let strong_mode = PasswordMode {
        title: "Strong password estimate",
        prompt: "Enter a strong password to estimate only: ",
        charset_label: STRONG_CHARSET_LABEL,
        max_length: MAX_STRONG_PASSWORD_LENGTH,
        charset: STRONG_CHARSET,
        validator: |c| STRONG_CHARSET.contains(&(c as u8)),
        validation_message: "Password must only contain supported letters, digits, and symbols.",
        result_label: "Strong Password estimate",
    };

    let weak_rate = run_bruteforce_demo(&weak_mode);
    let moderate_rate = run_bruteforce_demo(&moderate_mode);

    let local_average_rate = match (weak_rate, moderate_rate) {
        (Some(weak), Some(moderate)) => (weak + moderate) / 2.0,
        (Some(weak), None) => weak,
        (None, Some(moderate)) => moderate,
        (None, None) => 0.0,
    };

    let strong_password = read_password_for_mode(&strong_mode);

    print_strong_estimate_screen(STRONG_CHARSET_LABEL, local_average_rate);

    match estimate_strong_password(&strong_password, STRONG_CHARSET, local_average_rate) {
        Some(estimate) => print_strong_estimate_result(&estimate),
        None => print_estimate_failed(),
    }
}