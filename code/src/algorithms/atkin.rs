use crate::export::export_to_csv;
use crate::progress::create_progress_bar;
use crate::stats::AlgorithmStats;
use std::time::{Duration, Instant};

pub fn run_sieve_of_atkin(run_until_stopped: bool) {
    let algorithm_name = String::from("Sieve of Atkin");
    let mut limit = 10_000_000; // Starting limit
    let mut largest_prime = 0u64;
    let mut num_primes_found = 0usize;
    let start_time = Instant::now();

    loop {
        let primes = sieve_of_atkin(limit);
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

fn sieve_of_atkin(limit: usize) -> Vec<usize> {
    let mut sieve = vec![false; limit + 1];
    let sqrt_limit = (limit as f64).sqrt() as usize + 1;

    let total_steps = (sqrt_limit * sqrt_limit) as u64;
    let pb = create_progress_bar(total_steps, &format!("Running Sieve of Atkin up to {}", limit));

    for x in 1..=sqrt_limit {
        for y in 1..=sqrt_limit {
            pb.inc(1);
            let n = 4 * x * x + y * y;
            if n <= limit && (n % 12 == 1 || n % 12 == 5) {
                sieve[n] ^= true;
            }

            let n = 3 * x * x + y * y;
            if n <= limit && n % 12 == 7 {
                sieve[n] ^= true;
            }

            if x > y {
                let n = 3 * x * x - y * y;
                if n <= limit && n % 12 == 11 {
                    sieve[n] ^= true;
                }
            }
        }
    }
    pb.finish_and_clear();

    for n in 5..=sqrt_limit {
        if sieve[n] {
            let n_squared = n * n;
            let mut k = n_squared;
            while k <= limit {
                sieve[k] = false;
                k += n_squared;
            }
        }
    }

    let mut primes = vec![2, 3];
    for n in 5..=limit {
        if sieve[n] {
            primes.push(n);
        }
    }
    primes
}

