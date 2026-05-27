// ================================
// Configuration constants
// Central values for demo character sets, labels, and password length limits.
// ================================

// Central configuration values for the RustPassLab demo.
// Keeping these values in one file helps prevent the program logic and
// displayed text from drifting apart.

// ================================
// Weak password demo settings
// Lowercase-only character set and length limit for the weak brute-force demo.
// ================================

pub const WEAK_CHARSET: &[u8] = b"abcdefghijklmnopqrstuvwxyz";
pub const WEAK_CHARSET_LABEL: &str = "lowercase a-z";
pub const MAX_WEAK_PASSWORD_LENGTH: usize = 5;

// ================================
// Moderate password demo settings
// Mixed-case letters and digits used by the moderate brute-force demo.
// ================================

pub const MODERATE_CHARSET: &[u8] =
    b"abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789";
pub const MODERATE_CHARSET_LABEL: &str = "lowercase a-z, uppercase A-Z, digits 0-9";
pub const MAX_MODERATE_PASSWORD_LENGTH: usize = 5;

// ================================
// Strong password estimator settings
// Larger character set and length limit used for average-case estimates.
// ================================

pub const STRONG_CHARSET: &[u8] =
    b"abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789!@#$%^&*()-_=+[]{};:,.<>?/|";
pub const STRONG_CHARSET_LABEL: &str =
    "lowercase a-z, uppercase A-Z, digits 0-9, and common symbols";
pub const MAX_STRONG_PASSWORD_LENGTH: usize = 8;