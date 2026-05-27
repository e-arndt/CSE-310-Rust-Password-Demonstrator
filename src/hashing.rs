// ================================
// Imports
// SHA-256 hashing tools used by the password demonstration.
// ================================

use sha2::{Digest, Sha256};

// ================================
// Password hashing
// Converts plaintext input into SHA-256 hashes for comparison.
// ================================

// Converts a password or guess into a SHA-256 hash string.
// The program compares hashes instead of comparing the plaintext password directly.
pub fn hash_password(input: &str) -> String {
    let mut hasher = Sha256::new();

    hasher.update(input.as_bytes());

    let result = hasher.finalize();

    format!("{:x}", result)
}