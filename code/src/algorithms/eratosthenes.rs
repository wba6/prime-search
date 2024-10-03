use crate::export::export_to_csv;
use crate::progress::create_progress_bar;
use crate::stats::AlgorithmStats;
use std::time::{Duration, Instant};

pub fn run_sieve_of_eratosthenes(run_until_stopped: bool) {
    let algorithm_name = String::from("Sieve of Eratosthenes");
    let mut limit = 10_000_000; // Starting limit
    let mut largest_prime = 0u64;
    let mut num_primes_found = 0usize;
    let start_time = Instant::now();

    loop {
        let primes = sieve_of_eratosthenes(limit);
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
        limit *= 2;

        println!(
            "Increased limit to {}. Continuing to find larger primes...",
            limit
        );
    }
}

fn sieve_of_eratosthenes(limit: usize) -> Vec<usize> {
    let mut sieve = vec![true; limit + 1];
    sieve[0] = false;
    if limit >= 1 {
        sieve[1] = false;
    }
    let sqrt_limit = (limit as f64).sqrt() as usize + 1;
    let pb = create_progress_bar(sqrt_limit as u64, &format!("Running Sieve of Eratosthenes up to {}", limit));

    for num in 2..=sqrt_limit {
        pb.inc(1);
        if sieve[num] {
            let mut multiple = num * num;
            while multiple <= limit {
                sieve[multiple] = false;
                multiple += num;
            }
        }
    }
    pb.finish_and_clear();

    sieve
        .iter()
        .enumerate()
        .filter_map(|(i, &is_prime)| if is_prime { Some(i) } else { None })
        .collect()
}

