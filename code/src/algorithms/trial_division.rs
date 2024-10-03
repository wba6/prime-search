use crate::export::export_to_csv;
use crate::stats::AlgorithmStats;
use std::time::{Duration, Instant};

pub fn run_trial_division(run_until_stopped: bool, time_limit: Option<f64>) {
    let algorithm_name = String::from("Trial Division");
    let mut start = 2; // Starting number
    let segment_size = 100_000; // Size of each segment
    let mut primes = Vec::new(); // List of all found primes
    let start_time = Instant::now();

    loop {
        // Check time limit before starting the segment
        if let Some(limit_seconds) = time_limit {
            if start_time.elapsed() >= Duration::from_secs_f64(limit_seconds) {
                println!("Time limit reached. Stopping execution.");
                break;
            }
        }

        let end = start + segment_size - 1;
        let (new_primes, largest_prime, time_up) =
            trial_division(start, end, start_time, time_limit);
        primes.extend_from_slice(&new_primes);

        let time_taken = start_time.elapsed();
        let num_primes_found = primes.len();

        let stats = AlgorithmStats {
            algorithm_name: algorithm_name.clone(),
            time_taken,
            largest_prime: largest_prime as u64,
            num_primes_found,
        };

        // Export to CSV with a specific filename
        let filename = format!(
            "{}_stats.csv",
            algorithm_name.replace(' ', "_").to_lowercase()
        );
        export_to_csv(&stats, &filename);

        if time_up {
            println!("Time limit reached during processing. Stopping execution.");
            break;
        }

        // Adjusted logic here
        if time_limit.is_none() && !run_until_stopped {
            // No time limit specified and -m flag not set; exit after one iteration
            break;
        }

        // Prepare for next segment
        start = end + 1;

        println!(
            "Processed up to {}. Continuing to find larger primes...",
            end
        );
    }
}


fn trial_division(
    start: usize,
    end: usize,
    start_time: Instant,
    time_limit: Option<f64>,
) -> (Vec<usize>, usize, bool) {
    let mut new_primes = Vec::new();
    let mut time_up = false;
    let pb = crate::progress::create_progress_bar(
        (end - start + 1) as u64,
        &format!("Running Trial Division from {} to {}", start, end),
    );

    for n in start..=end {
        if let Some(limit_seconds) = time_limit {
            if start_time.elapsed() >= Duration::from_secs_f64(limit_seconds) {
                pb.finish_and_clear();
                println!("Time limit reached during processing. Stopping execution.");
                time_up = true;
                break;
            }
        }
        pb.inc(1);
        if is_prime_trial_division(n) {
            new_primes.push(n);
        }
    }
    pb.finish_and_clear();

    let largest_prime = *new_primes.last().unwrap_or(&0);
    (new_primes, largest_prime, time_up)
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
