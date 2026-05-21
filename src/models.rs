pub struct CrackResult {
    pub password: String,
    pub attempts: u64,
    pub elapsed_seconds: f64,
}

impl CrackResult {
    pub fn guesses_per_second(&self) -> f64 {
        self.attempts as f64 / self.elapsed_seconds
    }
}