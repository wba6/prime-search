use crate::export::export_to_csv;
use crate::progress::create_progress_bar;
use crate::stats::AlgorithmStats;
use std::time::{Duration, Instant};

pub fn run_trial_division(run_until_stopped: bool) {
    let algorithm_name = String::from("Trial Division");
    let mut limit = 100_000; // Starting limit
    let mut largest_prime = 0u64;
    let mut num_primes_found = 0usize;
    let start_time = Instant::now();

    loop {
        let primes = trial_division(limit);
        largest_prime = *primes.last().unwrap_or(&0) as u64;
        num_primes_found = primes.len();

        let time_taken = start_time.elapsed();

        let stats = AlgorithmStats {
            algorithm_name: algorithm_name.clone(),
            time_taken,
            largest_prime,
            num_primes_found,
        };

        // Export to CSV with a specific filename
        let filename = format!("{}_stats.csv", algorithm_name.replace(' ', "_").to_lowercase());
        export_to_csv(&stats, &filename);

        if !run_until_stopped {
            break;
        }

        // Increase the limit for the next iteration
        limit += 100_000; // Increase by 100,000

        println!(
            "Increased limit to {}. Continuing to find larger primes...",
            limit
        );
    }
}

fn trial_division(limit: usize) -> Vec<usize> {
    let mut primes = Vec::new();
    let pb = create_progress_bar((limit - 2) as u64, &format!("Running Trial Division up to {}", limit));

    for n in 2..=limit {
        pb.inc(1);
        if is_prime_trial_division(n) {
            primes.push(n);
        }
    }
    pb.finish_and_clear();
    primes
}


fn is_prime_trial_division(n: usize) -> bool {
    if n <= 1 {
        return false;
    }
    let sqrt_n = (n as f64).sqrt() as usize + 1;
    for i in 2..sqrt_n {
        if n % i == 0 {
            return false;
        }
    }
    true
}
