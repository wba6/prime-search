use std::time::Duration;

pub struct AlgorithmStats {
    pub algorithm_name: String,
    pub time_taken: Duration,
    pub largest_prime: u64,
    pub num_primes_found: usize,
}
