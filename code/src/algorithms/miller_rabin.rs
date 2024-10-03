use crate::export::export_to_csv;
use crate::stats::AlgorithmStats;
use rand::Rng;
use std::time::{Duration, Instant};

pub fn run_miller_rabin(run_until_stopped: bool, time_limit: Option<f64>) {
    let algorithm_name = String::from("Miller-Rabin Primality Test");
    let mut start = 2u64; // Starting number
    let segment_size = 1_000; // Numbers to test in each iteration
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
            miller_rabin_test(start, end, start_time, time_limit);
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

        // Prepare for next segment
        start = end + 1;

        println!(
            "Processed up to {}. Continuing to find larger primes...",
            end
        );
    }
}


fn miller_rabin_test(
    start: u64,
    end: u64,
    start_time: Instant,
    time_limit: Option<f64>,
) -> (Vec<u64>, u64, bool) {
    let mut new_primes = Vec::new();
    let mut time_up = false;
    let pb = crate::progress::create_progress_bar(
        end - start + 1,
        &format!("Running Miller-Rabin from {} to {}", start, end),
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
        if miller_rabin(n, 5) {
            new_primes.push(n);
        }
    }
    pb.finish_and_clear();

    let largest_prime = *new_primes.last().unwrap_or(&0);
    (new_primes, largest_prime, time_up)
}

fn miller_rabin(n: u64, k: u32) -> bool {
    if n <= 3 {
        return n == 2 || n == 3;
    }
    if n % 2 == 0 {
        return false;
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
