// ================================
// Strong password estimate model
// Stores average-case brute-force estimate results for stronger passwords.
// ================================

pub struct StrongEstimate {
    pub password: String,
    pub charset_size: u64,
    pub estimated_attempts: u128,
    pub estimated_seconds: f64,
}

// ================================
// Strong password estimator
// Estimates average brute-force difficulty using charset size, password length, and local rate.
// ================================

pub fn estimate_strong_password(
    password: &str,
    charset: &[u8],
    guesses_per_second: f64,
) -> Option<StrongEstimate> {
    if password.is_empty() || guesses_per_second <= 0.0 {
        return None;
    }

    let base = charset.len() as u128;
    let password_bytes = password.as_bytes();

    for byte in password_bytes {
        charset.iter().position(|c| c == byte)?;
    }

    let total_combinations = base.pow(password_bytes.len() as u32);
    let estimated_attempts = total_combinations.div_ceil(2).max(1);
    let estimated_seconds = estimated_attempts as f64 / guesses_per_second;

    Some(StrongEstimate {
        password: password.to_string(),
        charset_size: charset.len() as u64,
        estimated_attempts,
        estimated_seconds,
    })
}

// ================================
// Duration formatter
// Converts estimated seconds into a readable time unit for CLI and web output.
// ================================

pub fn format_duration(seconds: f64) -> String {
    let minutes = seconds / 60.0;
    let hours = minutes / 60.0;
    let days = hours / 24.0;
    let years = days / 365.25;

    if seconds < 60.0 {
        format!("{:.2} seconds", seconds)
    } else if minutes < 60.0 {
        format!("{:.2} minutes", minutes)
    } else if hours < 24.0 {
        format!("{:.2} hours", hours)
    } else if days < 365.25 {
        format!("{:.2} days", days)
    } else {
        format!("{:.2} years", years)
    }
}