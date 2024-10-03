use crate::export::export_to_csv;
use crate::stats::AlgorithmStats;
use std::time::{Duration, Instant};

pub fn run_sieve_of_eratosthenes(run_until_stopped: bool, time_limit: Option<f64>) {
    let algorithm_name = String::from("Sieve of Eratosthenes");
    let segment_size = 10_000_000; // Size of each segment
    let mut limit = segment_size;
    let mut primes = vec![2, 3]; // Initial primes
    let start_time = Instant::now();

    loop {
        // Check time limit before starting the segment
        if let Some(limit_seconds) = time_limit {
            if start_time.elapsed() >= Duration::from_secs_f64(limit_seconds) {
                println!("Time limit reached. Stopping execution.");
                break;
            }
        }

        let (new_primes, largest_prime, time_up) = segmented_sieve(
            limit - segment_size + 1,
            limit,
            &primes,
            start_time,
            time_limit,
        );

        primes.extend_from_slice(&new_primes);

        let time_taken = start_time.elapsed();
        let num_primes_found = primes.len();

        let stats = AlgorithmStats {
            algorithm_name: algorithm_name.clone(),
            time_taken,
            largest_prime,
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

        // Increase the limit for the next segment
        limit += segment_size;

        println!(
            "Processed up to {}. Continuing to find larger primes...",
            limit
        );
    }
}


fn segmented_sieve(
    start: usize,
    end: usize,
    base_primes: &[usize],
    start_time: Instant,
    time_limit: Option<f64>,
) -> (Vec<usize>, u64, bool) {
    let mut is_prime = vec![true; end - start + 1];
    let mut time_up = false;

    // Use base primes to mark non-primes in the current segment
    for &prime in base_primes {
        if let Some(limit_seconds) = time_limit {
            if start_time.elapsed() >= Duration::from_secs_f64(limit_seconds) {
                time_up = true;
                break;
            }
        }

        let mut multiple = ((start + prime - 1) / prime) * prime;
        if multiple < start {
            multiple += prime;
        }
        while multiple <= end {
            if let Some(limit_seconds) = time_limit {
                if start_time.elapsed() >= Duration::from_secs_f64(limit_seconds) {
                    time_up = true;
                    break;
                }
            }

            is_prime[multiple - start] = false;
            multiple += prime;
        }
        if time_up {
            break;
        }
    }

    // Find new primes in the current segment
    let mut new_primes = Vec::new();
    for i in start..=end {
        if let Some(limit_seconds) = time_limit {
            if start_time.elapsed() >= Duration::from_secs_f64(limit_seconds) {
                time_up = true;
                break;
            }
        }

        if is_prime[i - start] {
            new_primes.push(i);
        }
    }

    let largest_prime = *new_primes.last().unwrap_or(&0) as u64;
    (new_primes, largest_prime, time_up)
}
