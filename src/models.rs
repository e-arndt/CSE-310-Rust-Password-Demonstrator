// Stores the final result of a successful brute-force search.
pub struct CrackResult {
    pub password: String,
    pub matched_hash: String,
    pub attempts: u64,
    pub elapsed_seconds: f64,
}

impl CrackResult {
    // Calculates the average number of guesses checked per second.
    pub fn guesses_per_second(&self) -> f64 {
        if self.elapsed_seconds == 0.0 {
            0.0
        } else {
            self.attempts as f64 / self.elapsed_seconds
        }
    }
}