use crate::export::export_to_csv;
use crate::progress::create_progress_bar;
use crate::stats::AlgorithmStats;
use rand::Rng;
use std::time::{Duration, Instant};

pub fn run_miller_rabin(run_until_stopped: bool) {
    let algorithm_name = String::from("Miller-Rabin Primality Test");
    let mut limit = 1_000_000_000_000u64; // Starting limit
    let mut largest_prime = 0u64;
    let mut num_primes_found = 0usize;
    let start_time = Instant::now();

    loop {
        let (largest_in_run, primes_found_in_run) = miller_rabin_test(limit);
        if largest_in_run > largest_prime {
            largest_prime = largest_in_run;
        }
        num_primes_found += primes_found_in_run;

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
        limit *= 10; // Increase the range of random numbers

        println!(
            "Increased limit to {}. Continuing to find larger primes...",
            limit
        );
    }
}

fn miller_rabin_test(limit: u64) -> (u64, usize) {
    let mut rng = rand::thread_rng();
    let mut largest_prime = 0u64;
    let mut num_primes_found = 0usize;
    let num_tests = 1000; // Number of candidates to test

    let pb = create_progress_bar(num_tests, &format!("Running Miller-Rabin Primality Test up to {}", limit));

    for _ in 0..num_tests {
        pb.inc(1);
        let n = rng.gen_range(limit / 2..limit);
        if miller_rabin(n, 5) {
            num_primes_found += 1;
            if n > largest_prime {
                largest_prime = n;
            }
        }
    }
    pb.finish_and_clear();
    (largest_prime, num_primes_found)
}
fn miller_rabin(n: u64, k: u32) -> bool {
    if n <= 2 || n % 2 == 0 {
        return n == 2;
    }

    let mut d = n - 1;
    let mut s = 0u32;
    while d % 2 == 0 {
        d /= 2;
        s += 1;
    }

    let mut rng = rand::thread_rng();

    'witness_loop: for _ in 0..k {
        let a = rng.gen_range(2..n - 1);
        let mut x = mod_pow(a, d, n);
        if x == 1 || x == n - 1 {
            continue 'witness_loop;
        }
        for _ in 0..s - 1 {
            x = mod_pow(x, 2, n);
            if x == n - 1 {
                continue 'witness_loop;
            }
        }
        return false;
    }
    true
}

fn mod_pow(base: u64, mut exponent: u64, modulus: u64) -> u64 {
    if modulus == 1 {
        return 0;
    }
    let modulus_u128 = modulus as u128;
    let mut result = 1u128;
    let mut base_u128 = base as u128 % modulus_u128;
    while exponent > 0 {
        if exponent % 2 == 1 {
            result = (result * base_u128) % modulus_u128;
        }
        exponent >>= 1;
        base_u128 = (base_u128 * base_u128) % modulus_u128;
    }
    result as u64
}
