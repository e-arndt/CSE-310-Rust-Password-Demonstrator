// Central configuration values for the RustPassLab demo.
// Keeping these values in one file helps prevent the program logic and
// displayed text from drifting apart.

pub const WEAK_CHARSET: &[u8] = b"abcdefghijklmnopqrstuvwxyz";
pub const WEAK_CHARSET_LABEL: &str = "lowercase a-z";
pub const MAX_WEAK_PASSWORD_LENGTH: usize = 5;

pub const MODERATE_CHARSET: &[u8] =
    b"abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789";
pub const MODERATE_CHARSET_LABEL: &str = "lowercase a-z, uppercase A-Z, digits 0-9";
pub const MAX_MODERATE_PASSWORD_LENGTH: usize = 5;

pub const STRONG_CHARSET: &[u8] =
    b"abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789!@#$%^&*()-_=+[]{};:,.<>?/|";
pub const STRONG_CHARSET_LABEL: &str =
    "lowercase a-z, uppercase A-Z, digits 0-9, and common symbols";
pub const MAX_STRONG_PASSWORD_LENGTH: usize = 8;