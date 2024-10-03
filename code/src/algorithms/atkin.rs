use crate::export::export_to_csv;
use crate::stats::AlgorithmStats;
use std::cmp::max;
use std::time::{Duration, Instant};

pub fn run_sieve_of_atkin(run_until_stopped: bool, time_limit: Option<f64>) {
    let algorithm_name = String::from("Sieve of Atkin");
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

        let (new_primes, largest_prime, time_up) =
            sieve_of_atkin(limit - segment_size + 1, limit, start_time, time_limit);

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

fn sieve_of_atkin(
    start: usize,
    limit: usize,
    start_time: Instant,
    time_limit: Option<f64>,
) -> (Vec<usize>, u64, bool) {
    let mut sieve = vec![false; limit - start + 1];
    let sqrt_limit = (limit as f64).sqrt() as usize + 1;
    let mut time_up = false;

    // Generate base primes up to sqrt_limit using Sieve of Eratosthenes
    let mut is_prime_small = vec![true; sqrt_limit + 1];
    if sqrt_limit >= 0 {
        is_prime_small[0] = false;
    }
    if sqrt_limit >= 1 {
        is_prime_small[1] = false;
    }
    for i in 2..=sqrt_limit {
        if is_prime_small[i] {
            let mut multiple = i * i;
            while multiple <= sqrt_limit {
                is_prime_small[multiple] = false;
                multiple += i;
            }
        }
    }
    let base_primes: Vec<usize> = (2..=sqrt_limit).filter(|&i| is_prime_small[i]).collect();

    // Process the sieve
    for x in 1..=sqrt_limit {
        if let Some(limit_seconds) = time_limit {
            if start_time.elapsed() >= Duration::from_secs_f64(limit_seconds) {
                time_up = true;
                break;
            }
        }

        for y in 1..=sqrt_limit {
            if let Some(limit_seconds) = time_limit {
                if start_time.elapsed() >= Duration::from_secs_f64(limit_seconds) {
                    time_up = true;
                    break;
                }
            }

            let mut n = 4 * x * x + y * y;
            if n >= start && n <= limit && (n % 12 == 1 || n % 12 == 5) {
                sieve[n - start] ^= true;
            }

            n = 3 * x * x + y * y;
            if n >= start && n <= limit && n % 12 == 7 {
                sieve[n - start] ^= true;
            }

            if x > y {
                n = 3 * x * x - y * y;
                if n >= start && n <= limit && n % 12 == 11 {
                    sieve[n - start] ^= true;
                }
            }
        }
        if time_up {
            break;
        }
    }

    // Mark multiples of squares of base primes
    for &p in &base_primes {
        let p_squared = p * p;
        let mut k = ((start + p_squared - 1) / p_squared) * p_squared;
        while k <= limit {
            sieve[k - start] = false;
            k += p_squared;
        }
    }

    // Collect primes
    let mut new_primes = Vec::new();
    for n in start..=limit {
        if let Some(limit_seconds) = time_limit {
            if start_time.elapsed() >= Duration::from_secs_f64(limit_seconds) {
                time_up = true;
                break;
            }
        }

        if n == 2 || n == 3 {
            new_primes.push(n);
        } else if n > 3 && sieve[n - start] {
            new_primes.push(n);
        }
    }

    let largest_prime = *new_primes.last().unwrap_or(&0) as u64;
    (new_primes, largest_prime, time_up)
}

